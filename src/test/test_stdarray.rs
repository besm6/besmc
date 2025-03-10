use crate::*;
use crate::test::*;

//
// Compile file stdarray.std into hello_std.exe
//
// File stdarray.std can be created from examples/hello.pascal
// by commands:
//      pascompl -P hello.pascal stdarray.std
//
#[test]
fn test_stdarray_exe() {
    let options = CompilerOptions {
        files: vec!["examples/stdarray.std".to_string()],
        output_file: Some("target/hello_std.exe".to_string()),
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/hello_std.lst", " ДЛИHA БИБЛИOTEKИ"), " ДЛИHA БИБЛИOTEKИ  002 17");
}

//
// Compile file stdarray.std into lib_stdarray.obj
//
#[test]
fn test_stdarray_obj() {
    let options = CompilerOptions {
        files: vec!["examples/stdarray.std".to_string()],
        output_file: Some("target/lib_stdarray.obj".to_string()),
        stop_at_object: true,
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with("target/lib_stdarray.lst", " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY 0001 ЗOH.");
}
