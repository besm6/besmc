use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use std::process::Stdio;

use super::{CompilerOptions, FileGroups};

const HEADER: &str = "*name compile\n\
                      *disc:1/local\n\
                      *file:output,60,w\n";

const FOOTER_OBJ: &str = "*to perso: 60\n\
                          *end file\n";

const FOOTER_EXE: &str = "*call overlay\n\
                          program\n\
                          *end record\n\
                          *end file\n";

// Writes the contents of a source file to an already opened destination file
fn write_file_contents(mut dest_file: &fs::File, src_filename: &str) -> Result<(), String> {
    let mut src_file = fs::File::open(src_filename)
                                .map_err(|e| format!("Failed to open source file '{}': {}", src_filename, e))?;
    io::copy(&mut src_file, &mut dest_file)
       .map_err(|e| format!("Failed to copy to destination: {}", e))?;
    Ok(())
}

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
    let output_file = Path::new(&options.files[0]).file_stem().unwrap().to_str().unwrap().to_string() + ".exe";
    let listing_file = Path::new(&output_file).file_stem().unwrap().to_str().unwrap().to_string() + ".lst";

    // Create script for Dubna.
    let mut script = fs::File::create("build.dub")
                              .map_err(|e| format!("Failed to create build.dub: {}", e))?;

    // Write Dubna script.
    script.write_all(HEADER.as_bytes())
          .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // Write contents of each source file
    for src in &file_groups.ftn_files {
        script
            .write_all("*ftn\n".as_bytes())
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
        write_file_contents(&script, src)?;
    }
    //TODO: fortran
    //TODO: forex
    //TODO: algol
    //TODO: pascal
    //TODO: assem
    //TODO: madlen
    //TODO: bemsh
    //TODO: obj

    // Write the final step.
    if options.stop_at_object {
        // Save as library of object files.
        script.write_all(FOOTER_OBJ.as_bytes())
              .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    } else {
        // Create executable binary (overlay).
        script.write_all(FOOTER_EXE.as_bytes())
              .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    }

    // Ensure the file is written to disk
    script.flush()
          .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // Write listing to file.
    let listing = fs::File::create(&listing_file)
                           .map_err(|e| format!("Failed to create {}: {}", listing_file, e))?;

    // Run Dubna.
    let status = Command::new("dubna")
                         .arg("build.dub")
                         .stdout(Stdio::from(listing))
                         .status()
                         .map_err(|e| format!("Failed to execute dubna: {}", e))?;

    if status.success() {
        // Copy output.bin to output_file.
        let output = fs::File::create(&output_file)
                              .map_err(|e| format!("Failed to create {}: {}", output_file, e))?;
        write_file_contents(&output, "output.bin")?;
        remove_file("output.bin")?;
        Ok(())
    } else {
        Err(format!("Dubna failed with status: {}", status))
    }
}
