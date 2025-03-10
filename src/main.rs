use clap::Parser;
use std::panic;

mod compiler;
use compiler::compile_files;

#[cfg(test)]
mod test;

// Data structure to hold all parsed compiler options and files
#[derive(Debug, Parser, Default)]
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

    /// Keep intermediate files
    #[arg(short = 't', long = "save-temps")]
    save_temps: bool,

    // Input files
    #[arg(
        value_name = "FILES",
        help = "Input sources and object files:\n\
                 *.ftn     - Fortran-ГДP\n\
                 *.fortran - Fortran Dubna\n\
                 *.forex   - Forex\n\
                 *.algol   - Algol-ГДP\n\
                 *.pascal  - Pascal\n\
                 *.pas     - Pascal-re\n\
                 *.assem   - Assembler Madlen\n\
                 *.madlen  - Assembler Madlen-3.5\n\
                 *.bemsh   - Assembler БЕМШ\n\
                 *.obj     - Object Library (*perso)"
    )]
    files: Vec<String>,
}

fn main() {
    // Set empty panic hook.
    panic::set_hook(Box::new(|_| {}));

    // Use catch_unwind to handle fatal errors.
    let result = panic::catch_unwind(|| {

        // Parse arguments using clap
        let options = CompilerOptions::parse();

        // Print the parsed options for debug
        //println!("Options: {:#?}", options);

        compile_files(&options)
    });

    if let Err(panic_err) = result {
        // Extract and print the panic message
        if let Some(msg) = panic_err.downcast_ref::<String>() {
            eprintln!("{}", msg);
        } else if let Some(msg) = panic_err.downcast_ref::<&str>() {
            eprintln!("{}", msg);
        } else {
            eprintln!("Unknown error occurred");
        }
        std::process::exit(1); // Exit with error code
    }
}
