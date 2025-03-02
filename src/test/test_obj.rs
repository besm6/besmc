use crate::*;
use crate::test::*;

fn compile_obj(source_file_name: &str, output_file_name: &str) {
    let output_path = std::path::Path::new(&output_file_name);
    let listing_file = output_path.with_extension("lst").to_string_lossy().into_owned();

    let options = CompilerOptions {
        files: vec![source_file_name.to_string()],
        output_file: Some(output_file_name.to_string()),
        stop_at_object: true,
        ..Default::default()
    };
    compile_files(&options);

    assert_eq!(find_line_starting_with(&listing_file, " ДЛИHA LIBRARY"), " ДЛИHA LIBRARY 0001 ЗOH.");
}

#[test]
fn test_algol_obj() {
    compile_obj("examples/hello.algol", "target/lib_algol.obj");
}

#[test]
fn test_assem_obj() {
    compile_obj("examples/hello.assem", "target/lib_assem.obj");
}

#[test]
fn test_bemsh_obj() {
    compile_obj("examples/hello.bemsh", "target/lib_bemsh.obj");
}

#[test]
fn test_forex_obj() {
    compile_obj("examples/hello.forex", "target/lib_forex.obj");
}

#[test]
fn test_fortran_obj() {
    compile_obj("examples/hello.fortran", "target/lib_fortran.obj");
}

#[test]
fn test_ftn_obj() {
    compile_obj("examples/hello.ftn", "target/lib_ftn.obj");
}

#[test]
fn test_madlen_obj() {
    compile_obj("examples/hello.madlen", "target/lib_madlen.obj");
}

#[test]
fn test_pascal_obj() {
    compile_obj("examples/hello.pascal", "target/lib_pascal.obj");
}
