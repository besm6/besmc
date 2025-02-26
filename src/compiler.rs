use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use std::process::Stdio;

use super::{CompilerOptions, FileGroups};

//
// Writes the contents of a source file to an already opened destination file.
//
fn copy_file_contents(mut dest_file: &fs::File, src_filename: &str) -> Result<(), String> {
    let mut src_file = fs::File::open(src_filename)
                                .map_err(|e| format!("Failed to open source file '{}': {}", src_filename, e))?;
    io::copy(&mut src_file, &mut dest_file)
       .map_err(|e| format!("Failed to copy to destination: {}", e))?;
    Ok(())
}

//
// Writes the contents of many source files with given prefix
// to an already opened destination file.
//
fn copy_files(mut dest_file: &fs::File, list_of_files: &Vec<String>, prefix: &str) -> Result<(), String> {
    for src in list_of_files {
        dest_file.write_all(prefix.as_bytes())
              .map_err(|e| format!("Failed to write prefix: {}", e))?;
        copy_file_contents(&dest_file, src)?;
    }
    Ok(())
}

//
// Remove file and check status.
//
fn remove_file(filename: &str) -> Result<(), String> {
    match fs::remove_file(filename) {
        Ok(()) => {
            // File removed.
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // File not found, nothing to remove.
            Ok(())
        }
        Err(e) => Err(format!("Failed to remove '{}': {}", filename, e)),
    }
}

//
// Compiles files based on options and file groups.
// If -c is set, compile .ftn and .assem and everything else to .obj, then stop.
// Otherwise compile everything, then link into .exe.
//
pub fn compile_files(options: &CompilerOptions, file_groups: &FileGroups) -> Result<(), String> {

    // The first source file defines names of output binary and listing.
    let stem = Path::new(&options.files[0]).file_stem().unwrap().to_str().unwrap().to_string();
    let output_file = stem.clone() + if options.stop_at_object { ".obj" } else { ".exe" };
    let listing_file = stem + ".lst";

    // Create script for Dubna.
    let mut script = fs::File::create("build.dub")
                              .map_err(|e| format!("Failed to create build.dub: {}", e))?;
    writeln!(script, "*name compile\n\
                      *disc:1/local\n\
                      *file:output,60,w")
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // TODO: for each .obj input, create *file:inputN.bin directive.
    // Copy each .obj file to local inputN.bin file.
    // Here N=40...59.

    // TODO: for each .obj input, add *perso:N directive.

    // Write contents of each source file
    copy_files(&script, &file_groups.ftn_files, "*ftn\n")?;
    copy_files(&script, &file_groups.fortran_files, "*fortran\n")?;
    copy_files(&script, &file_groups.forex_files, "*forex\n")?;
    copy_files(&script, &file_groups.algol_files, "*algol\n")?;
    copy_files(&script, &file_groups.pascal_files, "*pascal\n")?;
    copy_files(&script, &file_groups.assem_files, "*assem\n")?;
    copy_files(&script, &file_groups.madlen_files, "*madlen\n")?;
    copy_files(&script, &file_groups.bemsh_files, "*bemsh\n")?;

    // Write the final step.
    if options.stop_at_object {
        // Save as library of object files.
        writeln!(script, "*to perso: 60\n\
                          *end file")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    } else {
        // Create executable binary (overlay).
        let entry = if file_groups.bemsh_files.is_empty() { "program" } else { "main" };
        writeln!(script, "*library:22\n\
                          *call overlay\n\
                          {}\n\
                          *end record\n\
                          *end file", entry)
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    }

    // Ensure the file is written to disk
    script.flush()
          .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    drop(script);

    // Write listing to file.
    let listing = fs::File::create(&listing_file)
                           .map_err(|e| format!("Failed to create {}: {}", listing_file, e))?;

    // Run Dubna.
    let status = Command::new("dubna")
                         .arg("build.dub")
                         .stdout(Stdio::from(listing))
                         .status()
                         .map_err(|e| format!("Failed to execute dubna: {}", e))?;

    //TODO: Scan listing and find compilation errors.

    if status.success() {
        // Copy output.bin to output_file.
        let output = fs::File::create(&output_file)
                              .map_err(|e| format!("Failed to create {}: {}", output_file, e))?;

        //TODO: add she-bang line "#!/usr/bin/env dubna"
        copy_file_contents(&output, "output.bin")?;
        remove_file("output.bin")?;
        remove_file("build.dub")?;
        //TODO: make output file executable
        Ok(())
    } else {
        Err(format!("Dubna failed with status: {}", status))
    }
}
