use crate::*;
use crate::test::*;

#[test]
fn test_algol_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_algol.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.algol".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_algol.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 17");
}

#[test]
fn test_assem_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_assem.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.assem".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_assem.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_bemsh_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_bemsh.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.bemsh".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_bemsh.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_forex_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_forex.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.forex".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_forex.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_fortran_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_fortran.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.fortran".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_fortran.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_ftn_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_ftn.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.ftn".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_ftn.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_madlen_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_madlen.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.madlen".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_madlen.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_pascal_exe() {
    let options = CompilerOptions {
        output_file: Some("hello_pascal.exe".to_string()),
        stop_at_object: false,
        files: vec!["examples/hello.pascal".to_string()],
    };
    compile_files(&options).expect("Compilation failed");

    assert_eq!(find_line_starting_with("hello_pascal.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 17");
}
