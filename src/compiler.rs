use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::process::Command;
use std::path::Path;
use std::process::Stdio;
use std::os::unix::fs::PermissionsExt;
use regex::Regex;

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
// Add *file:persoNN directive to the script.
// Here NN=40...57 (octal).
// Copy .obj file to a temporary 'persNN.bin' file.
// Increment perso index.
// Return name of the temporary file.
//
fn create_perso_file(mut script_file: &fs::File, obj_filename: &str, perso_index: &mut i32) -> String
{
    if *perso_index >= 0o60 {
        panic!("Cannot process {}: too many object files", obj_filename);
    }
    writeln!(script_file, "*file:pers{:o},{:o}", *perso_index, *perso_index)
        .unwrap_or_else(|e| { panic!("Failed to write *file: {}", e); });

    let bin_filename = format!("pers{:o}.bin", *perso_index);
    *perso_index += 1;

    // Copy file contents.
    let _ = fs::copy(obj_filename, &bin_filename)
               .unwrap_or_else(|e| { panic!("Failed to copy {} to {}: {}", obj_filename, bin_filename, e); });

    // Return name of temporary file.
    bin_filename
}

//
// Remove file and check status.
//
fn remove_file(filename: &str) {
    match fs::remove_file(filename) {
        Ok(()) => {}, // File removed
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}, // File not found, nothing to remove
        Err(e) => println!("Cannot remove '{}': {}", filename, e),
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

// Function to search for patterns in a file
fn search_errors_in_listing(file_path: &str, _options: &CompilerOptions) -> bool {

    // List of possible error messages
    let patterns = vec![
        String::from(r"БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ"),
        String::from(r"HET ′EOP′"),
        String::from(r"OTCYTCTBYET ИMЯ ПPOГPAMMЫ"),
        String::from(r"OTCYTCTBYET ИMЯ ПOДПPOГPAMMЫ"),
        String::from(r"OTCYTCTBYET ЗAГOЛOBOK ПOДПPOГPAMMЫ"),
        String::from(r"OTCYTCTBYET  PROGRAM"),
        String::from(r"ЗHAЧEH.* HE OПPEДEЛEHO"),
        String::from(r"INCORRECT ALGOL PROGRAM"),
        String::from(r"\*\*\*\*\*\*HEOПИCAHHЫЙ ИДEHTИФИKATOP"),
        String::from(r"\*\*\*\*\*\* HEOПИCAHHЫЙ ИДEHTИФИKATOP:"),
        String::from(r"^ \*\*\*\*\*\*\d+ "),
        String::from(r"^HEOП MET "),
        String::from(r"^ERROR \d+"),
        String::from(r"^ ERROR \d+"),
        String::from(r"OTCYTCTBYET"),
        String::from(r"HEДOПYCTИMЫЙ OПEPATOP:"),
        String::from(r"ДЛИHHЫЙ AДPEC B"),
    ];

    // Create a vector of compiled regex patterns
    let regexes: Vec<Regex> = patterns
        .into_iter()
        .map(|p| Regex::new(&p).expect("Invalid regex pattern"))
        .collect();

    // Open the file and create a buffered reader - panic on error
    let file = fs::File::open(file_path).unwrap_or_else(|e|
        panic!("Failed to open file {}: {}", file_path, e)
    );
    let reader = BufReader::new(file);

    // Flag to track if any matches were found
    let mut has_matches = false;

    // Process each line - panic on read error
    for (_line_num, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e|
            panic!("Failed to read line from {}: {}", file_path, e)
        );
        for regex in &regexes {
            if regex.is_match(&line) {
                println!("{}", line);
                has_matches = true;
                break; // Move to next line after first match
            }
        }
    }
    has_matches
}

//
// Compiles files based on options.
// If -c is set, compile .ftn and .assem and everything else to .obj, then stop.
// Otherwise compile everything, then link into .exe.
//
pub fn compile_files(options: &CompilerOptions) {

    // The first source file defines names of output binary and listing.
    let first_file = &options.files[0];
    let output_option = options.output_file.clone()
                               .unwrap_or(first_file.clone());
    let output_path = Path::new(&output_option);
    let output_extension = if options.stop_at_object { "obj" } else { "exe" };
    let output_file = output_path.with_extension(output_extension).to_string_lossy().into_owned();
    let listing_file = output_path.with_extension("lst").to_string_lossy().into_owned();
    let script_file = output_path.with_extension("dub").to_string_lossy().into_owned();

    // Create a list of files to remove when done.
    let mut files_to_remove: Vec<String> = vec![
        "output.bin".to_string(),
        script_file.clone(),
    ];

    // For each *.pas input, call pascompl and replace with *.std.
    let mut input_files = options.files.clone();
    for file in input_files.iter_mut() {
        if has_extension(&file, ".pas") {
            let path = Path::new(file);
            let std_file = path.with_extension("std").to_string_lossy().into_owned();

            // Run Pascal compiler.
            let status = Command::new("pascompl")
                                 .arg("-P")
                                 .arg(&file)
                                 .arg(&std_file)
                                 .status()
                                 .unwrap_or_else(|e| { panic!("Failed to execute pascompl: {}", e); });
            if !status.success() {
                panic!("Pascal compiler failed on {} with status: {}", file, status)
            }
            *file = std_file.clone();
            files_to_remove.push(std_file);
        }
    }

    // Create script for Dubna.
    let mut script = fs::File::create(&script_file)
                              .unwrap_or_else(|e| { panic!("Failed to create build.dub: {}", e); });
    writeln!(script, "*name compile\n\
                      *disc:1/local\n\
                      *file:output,60,w")
        .unwrap_or_else(|e| { panic!("Failed to write build.dub: {}", e); });

    // Add *file:persNN directive for each .obj file.
    let mut perso_index = 0o40;
    for file in &input_files {
        let path = Path::new(file);
        if let Some(ext) = path.extension() {
            if ext.to_string_lossy().as_ref() == "obj" {
                files_to_remove.push(create_perso_file(&script, file, &mut perso_index));
            }
        }
    }

    // Set single-page listing mode.
    writeln!(script, "*call setftn:one,long")
        .unwrap_or_else(|e| { panic!("Failed to write build.dub: {}", e); });

    // Write contents of each source file
    perso_index = 0o40;
    for file in &input_files {
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
                "obj"     => {
                                writeln!(&script, "*call perso:{:o},cont", perso_index)
                                    .unwrap_or_else(|e| { panic!("Failed to write *perso: {}", e); });
                                perso_index += 1;
                            },
                "std"     => copy_file(&script, &file, ""),
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
        writeln!(script, "*call to perso: 60\n\
                          *end file")
            .unwrap_or_else(|e| { panic!("Failed to write {}: {}", script_file, e); });
    } else {
        // Create executable binary (overlay).
        let entry = if has_extension(&first_file, ".bemsh") { "main" } else { "program" };
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

    // Scan listing and find compilation errors.
    if search_errors_in_listing(&listing_file, &options) {
        panic!("---\nCompilation failed!\nSee details in {}", listing_file);
    }

    // Copy output.bin to output_file.
    let output = fs::File::create(&output_file)
                          .unwrap_or_else(|e| { panic!("Failed to create {}: {}", output_file, e); });
    if !options.stop_at_object {
        // Add shebang line.
        writeln!(&output, "#!/usr/bin/env dubna")
            .unwrap_or_else(|e| { panic!("Failed to write shebang: {}", e); });
    }
    copy_file_contents(&output, "output.bin");

    // Remove temporary files.
    if !options.save_temps {
        for file_name in files_to_remove {
            remove_file(&file_name);
        }
    }

    if !options.stop_at_object {
        // Make output file executable.
        drop(output);
        make_file_executable(&output_file);
    }
}
