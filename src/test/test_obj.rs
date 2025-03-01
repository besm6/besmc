use crate::*;
use crate::test::*;

#[test]
fn test_algol_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_algol.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.algol".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_algol.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_assem_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_assem.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.assem".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_assem.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_bemsh_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_bemsh.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.bemsh".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_bemsh.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_forex_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_forex.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.forex".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_forex.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_fortran_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_fortran.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.fortran".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_fortran.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_ftn_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_ftn.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.ftn".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_ftn.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_madlen_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_madlen.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.madlen".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_madlen.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}

#[test]
fn test_pascal_obj() {
    let options = CompilerOptions {
        output_file: Some("target/lib_pascal.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/hello.pascal".to_string()],
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_pascal.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY  003 ЗOH");
}
