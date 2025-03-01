use crate::test::*;

#[test]
fn test_output_and_ftn_files() {
    let args = vec!["besmc", "-o", "out", "test.ftn", "main.ftn"];
    let result = parse_and_process(args);

    match result {
        Ok(options) => {
            assert_eq!(options.output_file, Some("out".to_string()));
            assert_eq!(options.stop_at_object, false);
            assert_eq!(options.files, vec![
                "test.ftn".to_string(),
                "main.ftn".to_string()]);
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}

#[test]
fn test_mixed_file_types() {
    let args = vec!["besmc", "-c", "src.ftn", "code.assem", "obj.obj"];
    let result = parse_and_process(args);

    match result {
        Ok(options) => {
            assert_eq!(options.output_file, None);
            assert_eq!(options.stop_at_object, true);
            assert_eq!(options.files, vec![
                "src.ftn".to_string(),
                "code.assem".to_string(),
                "obj.obj".to_string()]);
        }
        Err(msg) => panic!("Unexpected panic: {}", msg),
    }
}
