use std::path::Path;
use std::process::{Command, Stdio};

use super::{CompilerOptions, FileGroups};

// Runs a command and returns an error message if it fails
pub(crate) fn run_command(cmd: &mut Command) -> Result<(), String> {
    let output = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Command failed with status: {}", output.status))
    }
}

// Compiles files based on options and file groups
pub fn compile_files(options: &CompilerOptions, file_groups: &FileGroups) -> Result<(), String> {
    let output_file = options.output_file.as_deref().unwrap_or("a.out");

    // If -S is set, compile .c files to .s files and stop
    if options.stop_at_assembly {
        for c_file in &file_groups.c_files {
            let stem = Path::new(c_file).file_stem().unwrap().to_str().unwrap();
            let asm_file = format!("{}.s", stem);
            println!("Compiling {} to {}", c_file, asm_file);
            run_command(
                Command::new("gcc")
                    .args(&["-S", c_file, "-o", &asm_file])
            )?;
        }
        return Ok(()); // Stop here
    }

    // If -c is set, compile .c to .o and .s to .o, then stop
    if options.stop_at_object {
        let mut obj_files = Vec::new();
        for c_file in &file_groups.c_files {
            let stem = Path::new(c_file).file_stem().unwrap().to_str().unwrap();
            let obj_file = format!("{}.o", stem);
            println!("Compiling {} to {}", c_file, obj_file);
            run_command(
                Command::new("gcc")
                    .args(&["-c", c_file, "-o", &obj_file])
            )?;
            obj_files.push(obj_file);
        }
        for s_file in &file_groups.asm_files {
            let stem = Path::new(s_file).file_stem().unwrap().to_str().unwrap();
            let obj_file = format!("{}.o", stem);
            println!("Assembling {} to {}", s_file, obj_file);
            run_command(
                Command::new("gcc")
                    .args(&["-c", s_file, "-o", &obj_file])
            )?;
            obj_files.push(obj_file);
        }
        return Ok(()); // Stop here
    }

    // Full compilation: .c to .o, .s to .o, then link with existing .o files
    let mut obj_files = file_groups.obj_files.clone();
    for c_file in &file_groups.c_files {
        let stem = Path::new(c_file).file_stem().unwrap().to_str().unwrap();
        let obj_file = format!("{}.o", stem);
        println!("Compiling {} to {}", c_file, obj_file);
        run_command(
            Command::new("gcc")
                .args(&["-c", c_file, "-o", &obj_file])
        )?;
        obj_files.push(obj_file);
    }
    for s_file in &file_groups.asm_files {
        let stem = Path::new(s_file).file_stem().unwrap().to_str().unwrap();
        let obj_file = format!("{}.o", stem);
        println!("Assembling {} to {}", s_file, obj_file);
        run_command(
            Command::new("gcc")
                .args(&["-c", s_file, "-o", &obj_file])
        )?;
        obj_files.push(obj_file);
    }

    // Link all object files into the final executable
    if !obj_files.is_empty() {
        println!("Linking to {}", output_file);
        let mut cmd = Command::new("gcc");
        cmd.arg("-o").arg(output_file);
        for obj_file in &obj_files {
            cmd.arg(obj_file);
        }
        run_command(&mut cmd)?;
    }

    Ok(())
}
