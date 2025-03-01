use crate::*;
use crate::test::*;

fn compile_exe_negative(contents: &str, source_file: &str) {

    std::fs::write(source_file, contents).expect("Cannot write file");

    let output_file = std::path::Path::new(&source_file).with_extension("exe").to_string_lossy().into_owned();
    let options = CompilerOptions {
        output_file: Some(output_file.to_string()),
        stop_at_object: false,
        files: vec![source_file.to_string()],
    };

    // Try to compile and make sure it panicked.
    let result = panic::catch_unwind(|| {
        compile_files(&options);
    });
    assert!(result.is_err(), "Compilation did not fail");
}

#[test]
fn test_algol_bad_program() {
    let contents = "'begin'
    badprint(''Hello, Algol!'');
'end'
'eop'
";
    compile_exe_negative(contents, "target/algol_bad_program.algol");

    // Error messages:
    // * * INCORRECT ALGOL PROGRAM * *
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_algol_no_eop() {
    let contents = "foo
bar
";
    compile_exe_negative(contents, "target/algol_no_eop.algol");

    // Error message:
    // HET ′EOP′ !
}

#[test]
fn test_assem_no_header() {
    let contents = "        ,end,
";
    compile_exe_negative(contents, "target/assem_no_header.assem");

    // Error message:
    // OTCYTCTBYET ЗAГOЛOBOK ПOДПPOГPAMMЫ
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_assem_undefined_identifier() {
    let contents = " program: ,name,
        ,uj, foobar
        ,end,
";
    compile_exe_negative(contents, "target/assem_undefined_identifier.assem");

    // Error messages:
    // ******HEOПИCAHHЫЙ ИДEHTИФИKATOP           FOOBAR
    // ЧИCЛO ПEPФ. 0003      ЧИCЛO OШИБ. OПEPATOPOB  0001
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_madlen_no_header() {
    let contents = "        ,end,
";
    compile_exe_negative(contents, "target/madlen_no_header.madlen");

    // Error message:
    // OTCYTCTBYET ИMЯ ПPOГPAMMЫ
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_madlen_undefined_identifier() {
    let contents = " program: ,name,
        ,uj, foobar
        ,end,
";
    compile_exe_negative(contents, "target/madlen_undefined_identifier.madlen");

    // Error messages:
    // ****** HEOПИCAHHЫЙ ИДEHTИФИKATOP: ..... FOOBAR
    // ЧИCЛO ПEPФ.     03      ЧИCЛO OШИБ. OПEPATOPOB  01
    // OШИБKИ HA CTP.    1.
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_bemsh_no_header() {
    let contents = "ввд$$$
        э74
        финиш
квч$$$
трн$$$
кнц$$$
";
    compile_exe_negative(contents, "target/bemsh_no_header.bemsh");

    // Error message:
    // OTCYTCTBYET ИMЯ ПOДПPOГPAMMЫ
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_bemsh_undefined_identifier() {
    let contents = "ввд$$$
main    старт   512
        пб      куда
        финиш
квч$$$
трн$$$
кнц$$$
";
    compile_exe_negative(contents, "target/bemsh_undefined_identifier.bemsh");

    // Error messages:
    // HEOП MET
    // ЧИCЛO OШИБOK=0001. MAKC CEPЬEЗH=4.
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}
