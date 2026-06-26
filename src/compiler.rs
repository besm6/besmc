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

//
// Base directories where the BESM-6 C toolchain (headers and libc) is installed.
// Probed in order; the first existing match wins.
//
fn besm6_share_dirs() -> Vec<String> {
    let mut dirs = Vec::new();
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(format!("{}/.local/share/besm6", home));
    }
    dirs.push("/usr/local/share/besm6".to_string());
    dirs.push("/usr/share/besm6".to_string());
    dirs
}

//
// Locate the directory with BESM-6 C header files (*.h).
// Panic with a helpful message if not found.
//
fn find_besm6_include_dir() -> String {
    let mut probed = Vec::new();
    for base in besm6_share_dirs() {
        let dir = format!("{}/include", base);
        if Path::new(&dir).is_dir() {
            return dir;
        }
        probed.push(dir);
    }
    panic!("BESM-6 C headers not found. Looked in:\n  {}", probed.join("\n  "));
}

//
// Locate the BESM-6 libc binary (libc.bin).
// Panic with a helpful message if not found.
//
fn find_libc_path() -> String {
    let mut probed = Vec::new();
    for base in besm6_share_dirs() {
        let lib = format!("{}/lib/libc.bin", base);
        if Path::new(&lib).is_file() {
            return lib;
        }
        probed.push(lib);
    }
    panic!("BESM-6 libc.bin not found. Looked in:\n  {}", probed.join("\n  "));
}

//
// Run a compiler pass and panic if it fails.
//
fn run_pass(program: &str, args: &[&str]) {
    let status = Command::new(program)
                         .args(args)
                         .status()
                         .unwrap_or_else(|e| { panic!("Failed to execute {}: {}", program, e); });
    if !status.success() {
        panic!("{} failed with status: {}", program, status);
    }
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

    // Remember whether any C source is present: after pre-processing below, the C
    // files become *.madlen and can no longer be told apart from hand-written ones,
    // but their presence decides whether libc must be linked in.
    let has_c_files = options.files.iter().any(|f| has_extension(f, ".c"));

    // For each *.c input, run the C compiler pipeline and replace with *.madlen:
    //   1. cpp    -E -nostdinc -I <include_dir>  xxx.c   -> xxx.i
    //   2. b6parse                               xxx.i   -> xxx.asn
    //   3. b6lower                               xxx.asn -> xxx.tac
    //   4. b6codegen                             xxx.tac -> xxx.madlen
    // The resulting *.madlen is then assembled through the usual *madlen path.
    let mut include_dir = String::new();
    for file in input_files.iter_mut() {
        if has_extension(&file, ".c") {
            if include_dir.is_empty() {
                include_dir = find_besm6_include_dir();
            }
            // Derive intermediate names by appending to the full source name
            // (e.g. hello.c -> hello.c.i ... hello.c.madlen). This keeps the final
            // ".madlen" extension so it routes through the normal *madlen path, while
            // never clobbering an unrelated hand-written "hello.madlen" next to it.
            let i_file      = format!("{}.i", file);
            let asn_file    = format!("{}.asn", file);
            let tac_file    = format!("{}.tac", file);
            let madlen_file = format!("{}.madlen", file);

            // Use the traditional positional "cpp [options] infile outfile" form with a
            // joined -I<dir>: when /usr/bin/cpp is clang (e.g. macOS), the -o and spaced
            // -I forms are misparsed, while this form works on both GNU cpp and clang.
            let inc_flag = format!("-I{}", include_dir);
            run_pass("cpp", &["-E", "-nostdinc", &inc_flag, &file, &i_file]);
            run_pass("b6parse", &[&i_file, &asn_file]);
            run_pass("b6lower", &[&asn_file, &tac_file]);
            run_pass("b6codegen", &[&tac_file, &madlen_file]);

            *file = madlen_file.clone();
            files_to_remove.push(i_file);
            files_to_remove.push(asn_file);
            files_to_remove.push(tac_file);
            files_to_remove.push(madlen_file);
        }
    }

    // When linking C code, mount libc as a virtual disk file. The library lives
    // outside the working directory, so create a temporary symlink to it here
    // (dubna reads 'libc.bin' from the current directory for *file:libc).
    let link_libc = has_c_files && !options.stop_at_object;
    if link_libc {
        let libc_path = find_libc_path();
        remove_file("libc.bin");
        std::os::unix::fs::symlink(&libc_path, "libc.bin")
            .unwrap_or_else(|e| { panic!("Failed to create libc.bin symlink to {}: {}", libc_path, e); });
        files_to_remove.push("libc.bin".to_string());
    }

    // Create script for Dubna.
    let mut script = fs::File::create(&script_file)
                              .unwrap_or_else(|e| { panic!("Failed to create build.dub: {}", e); });
    writeln!(script, "*name compile\n\
                      *disc:1/local\n\
                      *file:output,60,w")
        .unwrap_or_else(|e| { panic!("Failed to write build.dub: {}", e); });

    // Mount the C runtime library when linking C code.
    if link_libc {
        writeln!(script, "*file:libc,37")
            .unwrap_or_else(|e| { panic!("Failed to write *file:libc: {}", e); });
    }

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

    // Add B language tape and library if any .b file is present.
    let has_b_files = input_files.iter().any(|f| has_extension(f, ".b"));
    if has_b_files {
        writeln!(script, "*tape:7/b,40\n\
                          *library:40")
            .unwrap_or_else(|e| { panic!("Failed to write build.dub: {}", e); });
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
                "b"       => copy_file(&script, &file, "*trans-main:40020\n"),
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
        // Search the C runtime library before the system library.
        if link_libc {
            writeln!(script, "*library:37")
                .unwrap_or_else(|e| { panic!("Failed to write *library:37: {}", e); });
        }
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
