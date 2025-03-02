use crate::*;
use crate::test::*;

#[test]
fn test_pascal_to_fortran() {

    // Compile main program written in Pascal.
    compile_files(&CompilerOptions {
        output_file: Some("target/caller.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/caller.pascal".to_string()],
        ..Default::default()
    });

    // Compile subroutine written in Fortran.
    compile_files(&CompilerOptions {
        output_file: Some("target/callee.obj".to_string()),
        stop_at_object: true,
        files: vec!["examples/callee.ftn".to_string()],
        ..Default::default()
    });

    // Link both together.
    compile_files(&CompilerOptions {
        output_file: Some("target/pascal_to_fortran.exe".to_string()),
        files: vec![
            "target/caller.obj".to_string(),
            "target/callee.obj".to_string(),
        ],
        ..Default::default()
    });

    assert_eq!(find_line_starting_with("target/pascal_to_fortran.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  004 03");
}
