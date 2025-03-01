use crate::*;
use crate::test::*;

#[test]
fn test_algol_negative() {
    let source_file = "target/wrong.algol";
    std::fs::write(source_file, "Line 1\nLine 2\nLine 3\n").expect("Cannot write file");

    let options = CompilerOptions {
        output_file: Some("target/wrong_algol.exe".to_string()),
        stop_at_object: false,
        files: vec![source_file.to_string()],
    };
    compile_files(&options).expect_err("Compilation did not fail");
}
