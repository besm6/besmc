use clap::Parser;

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

fn main() {
    // Parse arguments using clap
    let options = CompilerOptions::parse();

    // Print the parsed options and file groups for demonstration
    //println!("Options: {:#?}", options);

    compile_files(&options).expect("Compilation failed");
}
