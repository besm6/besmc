use crate::*;
use crate::test::*;

fn compile_exe_negative(contents: &str, source_file: &str, output_file: &str) {
    std::fs::write(source_file, contents).expect("Cannot write file");
    let options = CompilerOptions {
        output_file: Some(output_file.to_string()),
        stop_at_object: false,
        files: vec![source_file.to_string()],
    };
    compile_files(&options).expect_err("Compilation did not fail");
}

#[test]
fn test_algol_bad_program() {
    let contents = "'begin'
    badprint(''Hello, Algol!'');
'end'
'eop'
";
    compile_exe_negative(contents, "target/algol_bad_program.algol", "target/algol_bad_program.exe");

    // Error message: * * INCORRECT ALGOL PROGRAM * *
}

#[test]
fn test_algol_no_eop() {
    let contents = "foo
bar
";
    compile_exe_negative(contents, "target/algol_no_eop.algol", "target/algol_no_eop.exe");

    // Error message: HET ′EOP′ !
}
