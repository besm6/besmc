use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Helper to parse args and return options, or panic message
fn parse_and_process(args: Vec<&str>) -> Result<CompilerOptions, String> {
    let options = CompilerOptions::try_parse_from(args).unwrap_or_else(|e| {
        panic!("{}", e.to_string());
    });
    Ok(options)
}

//
// Scans a file for a line that starts with the given text.
// Returns the full line if found, empty string if not found.
//
fn find_line_starting_with(filename: &str, prefix: &str) -> String {
    // Open the file and create a buffered reader
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Scan each line in the file
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with(prefix) {
            return line; // Return the full matching line
        }
    }

    // No matching line found
    String::new() // Return empty string
}

#[test]
fn test_output_and_ftn_files() {
    let args = vec!["besmc", "-o", "out", "test.ftn", "main.ftn"];
    let result = parse_and_process(args);

    match result {
        Ok(options) => {
            assert_eq!(options.output_file, Some("out".to_string()));
            assert_eq!(options.stop_at_object, false);
            assert_eq!(options.files, vec![
                "test.ftn".to_string(),
                "main.ftn".to_string()]);
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}

#[test]
fn test_mixed_file_types() {
    let args = vec!["besmc", "-c", "src.ftn", "code.assem", "obj.obj"];
    let result = parse_and_process(args);

    match result {
        Ok(options) => {
            assert_eq!(options.output_file, None);
            assert_eq!(options.stop_at_object, true);
            assert_eq!(options.files, vec![
                "src.ftn".to_string(),
                "code.assem".to_string(),
                "obj.obj".to_string()]);
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}

#[test]
fn test_hello_algol() {
    let options = CompilerOptions {
        output_file: Some("hello_algol.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.algol".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_algol.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 17");
}

#[test]
fn test_hello_assem() {
    let options = CompilerOptions {
        output_file: Some("hello_assem.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.assem".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_assem.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_hello_bemsh() {
    let options = CompilerOptions {
        output_file: Some("hello_bemsh.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.bemsh".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_bemsh.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_hello_forex() {
    let options = CompilerOptions {
        output_file: Some("hello_forex.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.forex".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_forex.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_hello_fortran() {
    let options = CompilerOptions {
        output_file: Some("hello_fortran.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.fortran".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_fortran.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_hello_ftn() {
    let options = CompilerOptions {
        output_file: Some("hello_ftn.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.ftn".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_ftn.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_hello_madlen() {
    let options = CompilerOptions {
        output_file: Some("hello_madlen.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.madlen".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_madlen.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_hello_pascal() {
    let options = CompilerOptions {
        output_file: Some("hello_pascal.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.pascal".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_pascal.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 17");
}
