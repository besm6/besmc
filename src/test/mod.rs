use super::*;
use std::io::BufRead;

mod test_options;
mod test_exe;
mod test_obj;
mod test_negative;

// Helper to parse args and return options, or panic message
fn parse_and_process(args: Vec<&str>) -> Result<CompilerOptions, String> {
    let options = CompilerOptions::try_parse_from(args).unwrap_or_else(|e| {
        panic!("{}", e.to_string());
    });
    Ok(options)
}

//
// Scans a file for a line that starts with the given text.
// Returns the full line if found, empty string if not found.
//
fn find_line_starting_with(filename: &str, prefix: &str) -> String {
    // Open the file and create a buffered reader
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    // Scan each line in the file
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with(prefix) {
            return line; // Return the full matching line
        }
    }

    // No matching line found
    String::new() // Return empty string
}
