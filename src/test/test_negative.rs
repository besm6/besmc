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

#[test]
fn test_forex_undefined_variable() {
    let contents = "        program test
        a = b
        end
";
    compile_exe_negative(contents, "target/forex_undefined_variable.forex");

    // Error message:
    // ЗHAЧEH. A       HE ИCПOЛЬЗYETCЯ
    // ЗHAЧEH. B       HE OПPEДEЛEHO
}

#[test]
fn test_forex_undefined_label() {
    let contents = "        program test
        goto 123
        end
";
    compile_exe_negative(contents, "target/forex_undefined_label.forex");

    // Error messages:
    // ЗHAЧEH. 123  :  HE OПPEДEЛEHO
    // ЧИCЛO OШИБOK 0001
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_fortran_undefined_variable() {
    let contents = "        program test
        a = b
        end
";
    compile_exe_negative(contents, "target/fortran_undefined_variable.fortran");

    // Error message:
    // ******HEOПИCAHHЫЙ ИДEHTИФИKATOP           B
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_fortran_undefined_label() {
    let contents = "        program test
        goto 123
        end
";
    compile_exe_negative(contents, "target/fortran_undefined_label.fortran");

    // Error messages:
    // ******HEOПИCAHHЫЙ ИДEHTИФИKATOP           *123
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_ftn_undefined_variable() {
    let contents = "        program test
        a = b
        end
";
    compile_exe_negative(contents, "target/ftn_undefined_variable.ftn");

    // Error message:
    //           ERRORS: 1     WARNINGS: 0
    // ERROR 138          UNDEFINED IDENTIFIER>B<
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_ftn_undefined_label() {
    let contents = "        program test
        goto 123
        end
";
    compile_exe_negative(contents, "target/ftn_undefined_label.ftn");

    // Error messages:
    //           ERRORS: 1     WARNINGS: 0
    // ERROR 056 IN 00002 UNDEFINED LABEL >123<
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_pascal_missing_program() {
    let contents = "_(
    stop;
_).
";
    compile_exe_negative(contents, "target/pascal_missing_program.pascal");

    // Error message:
    // ******136  TPEБYETCЯ  PROGRAM
    // ******95  TPEБYETCЯ  ЛEBAЯ CKOБKA
    // ******100  TPEБYETCЯ  ПPABAЯ CKOБKA
    // ******77 HET OUTPUT
    // ******52 BCTPETИЛCЯ KOHEЦ ФAЙЛA
    // IN 3 LINES 5 ERRORS
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}

#[test]
fn test_pascal_undefined_variable() {
    let contents = "program main(output);
_(
    a = 123;
    stop;
_).
";
    compile_exe_negative(contents, "target/pascal_undefined_variable.pascal");

    // Error messages:
    // ******11 ИДEHTИФИKATOP HE OПPEДEЛEH
    // IN 5 LINES 1 ERRORS
    // БЫЛИ OШИБKИ ПPИ BBOДE ИЛИ TPAHCЛЯЦИИ !!!
}
