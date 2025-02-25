use super::*;
use std::panic;

// Helper to parse args and return options + file groups, or panic message
fn parse_and_process(args: Vec<&str>) -> Result<(CompilerOptions, FileGroups), String> {
    let options = CompilerOptions::try_parse_from(args).unwrap_or_else(|e| {
        panic!("{}", e.to_string());
    });

    let result = panic::catch_unwind(|| {
        options.validate();
        let file_groups = options.categorize_files();
        (options, file_groups)
    });

    result.map_err(|panic_err| {
        if let Some(msg) = panic_err.downcast_ref::<String>() {
            msg.clone()
        } else if let Some(msg) = panic_err.downcast_ref::<&str>() {
            msg.to_string()
        } else {
            "Unknown panic occurred".to_string()
        }
    })
}

#[test]
fn test_output_and_ftn_files() {
    let args = vec!["besmc", "-o", "out", "test.ftn", "main.ftn"];
    let result = parse_and_process(args);

    match result {
        Ok((options, file_groups)) => {
            assert_eq!(options.output_file, Some("out".to_string()));
            assert_eq!(options.stop_at_object, false);
            assert_eq!(
                file_groups,
                FileGroups {
                    ftn_files: vec!["test.ftn".to_string(), "main.ftn".to_string()],
                    fortran_files: vec![],
                    forex_files: vec![],
                    algol_files: vec![],
                    pascal_files: vec![],
                    assem_files: vec![],
                    madlen_files: vec![],
                    bemsh_files: vec![],
                    obj_files: vec![],
                }
            );
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}

#[test]
fn test_unknown_extension() {
    let args = vec!["besmc", "-o", "out", "test.xyz"];
    let result = parse_and_process(args);

    match result {
        Ok(_) => panic!("Expected panic due to unknown extension"),
        Err(msg) => assert_eq!(msg, "Cannot process file with unknown extension: test.xyz"),
    }
}

#[test]
fn test_mixed_file_types() {
    let args = vec!["besmc", "-c", "src.ftn", "code.assem", "obj.obj"];
    let result = parse_and_process(args);

    match result {
        Ok((options, file_groups)) => {
            assert_eq!(options.output_file, None);
            assert_eq!(options.stop_at_object, true);
            assert_eq!(
                file_groups,
                FileGroups {
                    ftn_files: vec!["src.ftn".to_string()],
                    fortran_files: vec![],
                    forex_files: vec![],
                    algol_files: vec![],
                    pascal_files: vec![],
                    assem_files: vec!["code.assem".to_string()],
                    madlen_files: vec![],
                    bemsh_files: vec![],
                    obj_files: vec!["obj.obj".to_string()],
                }
            );
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}
