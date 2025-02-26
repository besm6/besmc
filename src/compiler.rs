use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};

use super::{CompilerOptions, FileGroups};

// Writes the contents of a source file to an already opened destination file
fn write_file_contents(mut dest_file: &File, src_filename: &str) -> Result<(), String> {
    // Open the source file
    let mut src_file = File::open(src_filename)
        .map_err(|e| format!("Failed to open source file '{}': {}", src_filename, e))?;

    // Read the contents of the source file into a string
    let mut contents = String::new();
    src_file
        .read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read source file '{}': {}", src_filename, e))?;

    // Write the contents to the destination file
    dest_file
        .write_all(contents.as_bytes())
        .map_err(|e| format!("Failed to write to destination file: {}", e))?;

    Ok(())
}

//
// Compiles files based on options and file groups.
// If -c is set, compile .ftn and .assem and everything else to .obj, then stop.
// Otherwise compile everything, then link into .exe.
//
pub fn compile_files(options: &CompilerOptions, file_groups: &FileGroups) -> Result<(), String> {
    let _output_file = options.output_file.as_deref().unwrap_or("out.exe");

    // Create script for Dubna.
    let mut script = File::create("build.dub")
        .map_err(|e| format!("Failed to create build.dub: {}", e))?;

    // Write Dubna script.
    writeln!(script, "*name hello")
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    writeln!(script, "*disc:1/local")
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    writeln!(script, "*file:output,60,w")
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // Write contents of each source file
    for src in &file_groups.ftn_files {
        writeln!(script, "*ftn")
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
        writeln!(script, "*to perso: 60")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    } else {
        // Create executable binary (overlay).
        writeln!(script, "*call overlay")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
        writeln!(script, "program")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
        writeln!(script, "*end record")
            .map_err(|e| format!("Failed to write build.dub: {}", e))?;
    }
    writeln!(script, "*end file")
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // Ensure the file is written to disk
    script.flush()
        .map_err(|e| format!("Failed to write build.dub: {}", e))?;

    // Run Dubna.
    let status = Command::new("dubna")
        .arg("build.dub")
        .status()
        .map_err(|e| format!("Failed to execute dubna: {}", e))?;

    if status.success() {
        //TODO: copy/rename output.bin as output_file.
        Ok(())
    } else {
        Err(format!("Dubna failed with status: {}", status))
    }
}
