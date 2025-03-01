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
fn copy_file_contents(mut dest_file: &fs::File, src_filename: &str)  {
    let mut src_file = fs::File::open(src_filename)
                                .unwrap_or_else(|e| { panic!("Failed to open file '{}': {}", src_filename, e); });
    io::copy(&mut src_file, &mut dest_file)
       .unwrap_or_else(|e| { panic!("Failed to copy to destination: {}", e); });
}

//
// Writes the contents of many source files with given prefix
// to an already opened destination file.
//
fn copy_file(mut dest_file: &fs::File, src: &str, prefix: &str)  {
    dest_file.write_all(prefix.as_bytes())
             .unwrap_or_else(|e| { panic!("Failed to write prefix: {}", e); });
    copy_file_contents(&dest_file, src);
}

//
// Remove file and check status.
//
fn remove_file(filename: &str) {
    match fs::remove_file(filename) {
        Ok(()) => {}, // File removed
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}, // File not found, nothing to remove
        Err(e) => panic!("Failed to remove '{}': {}", filename, e),
    }
}

//
// Remove file and check status.
//
fn make_file_executable(file_path: &str) {

    // Get the current permissions of the file
    let metadata = fs::metadata(file_path)
                      .unwrap_or_else(|e| { panic!("Cannot get metadata for {}: {}", file_path, e); });
    let mut permissions = metadata.permissions();

    // Set executable bits.
    permissions.set_mode(permissions.mode() | 0o111);

    // Apply the new permissions to the file
    fs::set_permissions(file_path, permissions)
        .unwrap_or_else(|e| { panic!("Cannot set permissions for {}: {}", file_path, e); });
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
pub fn compile_files(options: &CompilerOptions) {

    // The first source file defines names of output binary and listing.
    let output_option = options.output_file.clone()
                               .unwrap_or(options.files[0].clone());
    let output_path = Path::new(&output_option);
    let output_extension = if options.stop_at_object { "obj" } else { "exe" };
    let output_file = output_path.with_extension(output_extension).to_string_lossy().into_owned();
    let listing_file = output_path.with_extension("lst").to_string_lossy().into_owned();
    let script_file = output_path.with_extension("dub").to_string_lossy().into_owned();

    // Create script for Dubna.
    let mut script = fs::File::create(&script_file)
                              .unwrap_or_else(|e| { panic!("Failed to create build.dub: {}", e); });
    writeln!(script, "*name compile\n\
                      *disc:1/local\n\
                      *file:output,60,w")
        .unwrap_or_else(|e| { panic!("Failed to write build.dub: {}", e); });

    // TODO: for each .obj input, create *file:inputN.bin directive.
    // Copy each .obj file to local inputN.bin file.
    // Here N=40...59.

    // Write contents of each source file
    for file in &options.files {
        let path = Path::new(file);
        if let Some(ext) = path.extension() {
            match ext.to_string_lossy().as_ref() {
                "ftn"     => copy_file(&script, &file, "*ftn\n"),
                "fortran" => copy_file(&script, &file, "*fortran\n"),
                "forex"   => copy_file(&script, &file, "*forex\n"),
                "algol"   => copy_file(&script, &file, "*algol\n"),
                "pascal"  => copy_file(&script, &file, "*pascal\n"),
                "assem"   => copy_file(&script, &file, "*assem\n"),
                "madlen"  => copy_file(&script, &file, "*madlen\n"),
                "bemsh"   => copy_file(&script, &file, "*bemsh\n"),
                "obj"     => {}, // TODO: for each .obj input, add *perso:N directive.
                "exe"     => panic!("Cannot process executable file: {}", file),
                _         => panic!("Unknown file extension: {}", file),
            }
        } else {
            panic!("Cannot process file without extension: {}", file);
        }
    }

    // Write the final step.
    if options.stop_at_object {
        // Save as library of object files.
        writeln!(script, "*to perso: 60\n\
                          *end file")
            .unwrap_or_else(|e| { panic!("Failed to write {}: {}", script_file, e); });
    } else {
        // Create executable binary (overlay).
        let entry = if has_extension(&options.files[0], ".bemsh") { "main" } else { "program" };
        writeln!(script, "*library:22\n\
                          *call overlay\n\
                          {}\n\
                          *end record\n\
                          *end file", entry)
            .unwrap_or_else(|e| { panic!("Failed to write {}: {}", script_file, e); });
    }

    // Ensure the file is written to disk
    script.flush()
          .unwrap_or_else(|e| { panic!("Failed to flush {}: {}", script_file, e); });
    drop(script);

    // Write listing to file.
    let listing = fs::File::create(&listing_file)
                           .unwrap_or_else(|e| { panic!("Failed to create {}: {}", listing_file, e); });

    // Run Dubna.
    let status = Command::new("dubna")
                         .arg(&script_file)
                         .stdout(Stdio::from(listing))
                         .status()
                         .unwrap_or_else(|e| { panic!("Failed to execute dubna: {}", e); });
    if !status.success() {
        panic!("Dubna failed with status: {}", status)
    }

    //TODO: Scan listing and find compilation errors.

    // Copy output.bin to output_file.
    let output = fs::File::create(&output_file)
                          .unwrap_or_else(|e| { panic!("Failed to create {}: {}", output_file, e); });
    if !options.stop_at_object {
        // Add shebang line.
        writeln!(&output, "#!/usr/bin/env dubna")
            .unwrap_or_else(|e| { panic!("Failed to write shebang: {}", e); });
    }
    copy_file_contents(&output, "output.bin");

    remove_file("output.bin");
    remove_file(&script_file);

    if !options.stop_at_object {
        // Make output file executable.
        drop(output);
        make_file_executable(&output_file);
    }
}
