use clap::Parser;
use std::path::Path;
use std::panic;

mod compiler;
use compiler::compile_files;

#[cfg(test)]
mod test;

// Data structure to hold all parsed compiler options and files
#[derive(Debug, Parser)]
#[command(
    about = "BESM-6 compiler frontend",
    disable_help_flag = false,
    arg_required_else_help = true
)]
struct CompilerOptions {
    /// Output file name
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_file: Option<String>,

    /// Compile only to object files
    #[arg(short = 'c', long = "compile")]
    stop_at_object: bool,

    /// Compile only to assembly files
    #[arg(short = 'S', long = "assembly")]
    stop_at_assembly: bool,

    /// Input files to process
    #[arg(value_name = "FILES")]
    files: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
struct FileGroups {
    c_files: Vec<String>,
    asm_files: Vec<String>,
    obj_files: Vec<String>,
}

impl CompilerOptions {
    // Process files into categorized groups
    fn categorize_files(&self) -> FileGroups {
        let mut groups = FileGroups::default();

        for file in &self.files {
            let path = Path::new(file);
            if let Some(ext) = path.extension() {
                match ext.to_string_lossy().as_ref() {
                    "c" => groups.c_files.push(file.clone()),
                    "s" => groups.asm_files.push(file.clone()),
                    "o" => groups.obj_files.push(file.clone()),
                    _ => panic!("Cannot process file with unknown extension: {}", file),
                }
            } else {
                panic!("Cannot process file without extension: {}", file);
            }
        }

        groups
    }

    // Validate options
    fn validate(&self) {
        if self.stop_at_object && self.stop_at_assembly {
            panic!("Options -c and -S cannot be used together");
        }
    }
}

fn main() {
    // Set a custom panic hook
    panic::set_hook(Box::new(|panic_info| {
        // Print location of the panic.
        if let Some(_location) = panic_info.location() {
            // Uncomment for debug:
            // println!("Aborted at {}, line {}:", location.file(), location.line());
        }
        // Proceed to catch_unwind().
    }));

    // Parse arguments using clap
    let options = CompilerOptions::parse();

    // Use catch_unwind to handle panics from validation and file categorization
    let result = panic::catch_unwind(|| {
        options.validate();
        let file_groups = options.categorize_files();

        // Print the parsed options and file groups for demonstration
        println!("Options: {:#?}", options);
        println!("File Groups: {:#?}", file_groups);

        compile_files(&options, &file_groups).expect("Compilation failed");
        ()
    });

    match result {
        Ok(()) => {
            // All good.
        }
        Err(panic_err) => {
            // Extract and print the panic message
            if let Some(msg) = panic_err.downcast_ref::<String>() {
                eprintln!("Error: {}", msg);
            } else if let Some(msg) = panic_err.downcast_ref::<&str>() {
                eprintln!("Error: {}", msg);
            } else {
                eprintln!("Unknown error occurred");
            }
            std::process::exit(1); // Exit with error code
        }
    }
}
