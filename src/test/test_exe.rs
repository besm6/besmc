use crate::*;
use crate::test::*;

#[test]
fn test_algol_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_algol.exe".to_string()),
        files: vec!["examples/hello.algol".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_algol.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 17");
}

#[test]
fn test_assem_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_assem.exe".to_string()),
        files: vec!["examples/hello.assem".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_assem.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_bemsh_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_bemsh.exe".to_string()),
        files: vec!["examples/hello.bemsh".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_bemsh.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_forex_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_forex.exe".to_string()),
        files: vec!["examples/hello.forex".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_forex.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_fortran_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_fortran.exe".to_string()),
        files: vec!["examples/hello.fortran".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_fortran.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_ftn_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_ftn.exe".to_string()),
        files: vec!["examples/hello.ftn".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_ftn.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 30");
}

#[test]
fn test_madlen_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_madlen.exe".to_string()),
        files: vec!["examples/hello.madlen".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_madlen.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  001 01");
}

#[test]
fn test_pascal_exe() {
    let options = CompilerOptions {
        output_file: Some("target/hello_pascal.exe".to_string()),
        files: vec!["examples/hello.pascal".to_string()],
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_pascal.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 17");
}
