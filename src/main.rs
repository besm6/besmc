use clap::Parser;
use std::panic;
use std::path::Path;

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

    // Input files
    #[arg(
        value_name = "FILES",
        help = "Input sources and object files:\n\
                 *.ftn     - Fortran-ГДP\n\
                 *.fortran - Fortran Dubna\n\
                 *.forex   - Forex\n\
                 *.algol   - Algol-ГДP\n\
                 *.pascal  - Pascal\n\
                 *.assem   - Assembler Madlen\n\
                 *.madlen  - Assembler Madlen-3.5\n\
                 *.bemsh   - Assembler БЕМШ\n\
                 *.obj     - Object Library (*perso)"
    )]
    files: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
struct FileGroups {
    ftn_files: Vec<String>,
    fortran_files: Vec<String>,
    forex_files: Vec<String>,
    algol_files: Vec<String>,
    pascal_files: Vec<String>,
    assem_files: Vec<String>,
    madlen_files: Vec<String>,
    bemsh_files: Vec<String>,
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
                    "ftn" => groups.ftn_files.push(file.clone()),
                    "fortran" => groups.fortran_files.push(file.clone()),
                    "forex" => groups.forex_files.push(file.clone()),
                    "algol" => groups.algol_files.push(file.clone()),
                    "pascal" => groups.pascal_files.push(file.clone()),
                    "assem" => groups.assem_files.push(file.clone()),
                    "madlen" => groups.madlen_files.push(file.clone()),
                    "bemsh" => groups.bemsh_files.push(file.clone()),
                    "obj" => groups.obj_files.push(file.clone()),
                    "exe" => panic!("Cannot process executable file: {}", file),
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
        //TODO
        //if self.stop_at_object && self.stop_at_assembly {
        //    panic!("Options -c and -S cannot be used together");
        //}
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
        //println!("Options: {:#?}", options);
        //println!("File Groups: {:#?}", file_groups);

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
