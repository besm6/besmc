use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use std::process::Stdio;
use std::os::unix::fs::PermissionsExt;

use super::{CompilerOptions};

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
fn copy_file(mut dest_file: &fs::File, src: &str, prefix: &str) -> Result<(), String> {
    dest_file.write_all(prefix.as_bytes())
          .map_err(|e| format!("Failed to write prefix: {}", e))?;
    copy_file_contents(&dest_file, src)?;
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
// Remove file and check status.
//
fn make_file_executable(file_path: &str) -> Result<(), String> {

    // Get the current permissions of the file
    let metadata = fs::metadata(file_path)
                      .map_err(|e| format!("Cannot get metadata for {}: {}", file_path, e))?;
    let mut permissions = metadata.permissions();

    // Set executable bits.
    permissions.set_mode(permissions.mode() | 0o111);

    // Apply the new permissions to the file
    fs::set_permissions(file_path, permissions)
       .map_err(|e| format!("Cannot set permissions for {}: {}", file_path, e))?;
    Ok(())
}

//
// Check if a file name has a given extension.
// Extension must start with a dot.
// Comparison is case-insensitive, e.g., ".PDF" and ".pdf" are treated the same.
// Returns true if the file has the extension.
//
fn has_extension(filename: &str, ext_with_dot: &str) -> bool {
    let ext_lower = ext_with_dot.to_lowercase();
    filename.to_lowercase().ends_with(&ext_lower)
}

//
// Compiles files based on options.
// If -c is set, compile .ftn and .assem and everything else to .obj, then stop.
// Otherwise compile everything, then link into .exe.
//
pub fn compile_files(options: &CompilerOptions) -> Result<(), String> {

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

    // Write contents of each source file
    for file in &options.files {
        let path = Path::new(file);
        if let Some(ext) = path.extension() {
            match ext.to_string_lossy().as_ref() {
                "ftn"     => copy_file(&script, &file, "*ftn\n")?,
                "fortran" => copy_file(&script, &file, "*fortran\n")?,
                "forex"   => copy_file(&script, &file, "*forex\n")?,
                "algol"   => copy_file(&script, &file, "*algol\n")?,
                "pascal"  => copy_file(&script, &file, "*pascal\n")?,
                "assem"   => copy_file(&script, &file, "*assem\n")?,
                "madlen"  => copy_file(&script, &file, "*madlen\n")?,
                "bemsh"   => copy_file(&script, &file, "*bemsh\n")?,
                "obj"     => {}, // TODO: for each .obj input, add *perso:N directive.
                "exe"     => return Err(format!("Cannot process executable file: {}", file)),
                _         => return Err(format!("Unknown file extension: {}", file)),
            }
        } else {
            return Err(format!("Cannot process file without extension: {}", file));
        }
    }

    // Write the final step.
    if options.stop_at_object {
        // Save as library of object files.
        writeln!(script, "*to perso: 60\n\
                          *end file")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    } else {
        // Create executable binary (overlay).
        let entry = if has_extension(&options.files[0], ".bemsh") { "main" } else { "program" };
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
        if !options.stop_at_object {
            // Add shebang line.
            writeln!(&output, "#!/usr/bin/env dubna")
                .map_err(|e| format!("Failed to write shebang: {}", e))?;
        }
        copy_file_contents(&output, "output.bin")?;
        remove_file("output.bin")?;
        remove_file("build.dub")?;
        if !options.stop_at_object {
            // Make output file executable.
            drop(output);
            make_file_executable(&output_file)?;
        }
        Ok(())
    } else {
        Err(format!("Dubna failed with status: {}", status))
    }
}
