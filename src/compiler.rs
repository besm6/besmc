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
    let output_file = options.output_file.as_deref().unwrap_or("out.exe");

    // If -c is set, compile .ftn to .obj and .assem to .obj, then stop
    if options.stop_at_object {
        let mut obj_files = Vec::new();
        for ftn_file in &file_groups.ftn_files {
            let stem = Path::new(ftn_file).file_stem().unwrap().to_str().unwrap();
            let obj_file = format!("{}.obj", stem);
            println!("Compiling {} to {}", ftn_file, obj_file);
            run_command(
                Command::new("dubna")
                    .args(&["-c", ftn_file, "-o", &obj_file])
            )?;
            obj_files.push(obj_file);
        }
        for assem_file in &file_groups.assem_files {
            let stem = Path::new(assem_file).file_stem().unwrap().to_str().unwrap();
            let obj_file = format!("{}.obj", stem);
            println!("Assembling {} to {}", assem_file, obj_file);
            run_command(
                Command::new("dubna")
                    .args(&["-c", assem_file, "-o", &obj_file])
            )?;
            obj_files.push(obj_file);
        }
        return Ok(()); // Stop here
    }

    // Full compilation: .ftn to .obj and .assem to .obj, then link with existing .obj files
    let mut obj_files = file_groups.obj_files.clone();
    for ftn_file in &file_groups.ftn_files {
        let stem = Path::new(ftn_file).file_stem().unwrap().to_str().unwrap();
        let obj_file = format!("{}.obj", stem);
        println!("Compiling {} to {}", ftn_file, obj_file);
        run_command(
            Command::new("dubna")
                .args(&["-c", ftn_file, "-o", &obj_file])
        )?;
        obj_files.push(obj_file);
    }
    for assem_file in &file_groups.assem_files {
        let stem = Path::new(assem_file).file_stem().unwrap().to_str().unwrap();
        let obj_file = format!("{}.obj", stem);
        println!("Assembling {} to {}", assem_file, obj_file);
        run_command(
            Command::new("dubna")
                .args(&["-c", assem_file, "-o", &obj_file])
        )?;
        obj_files.push(obj_file);
    }

    // Link all object files into the final executable
    if !obj_files.is_empty() {
        println!("Linking to {}", output_file);
        let mut cmd = Command::new("dubna");
        cmd.arg("-o").arg(output_file);
        for obj_file in &obj_files {
            cmd.arg(obj_file);
        }
        run_command(&mut cmd)?;
    }

    Ok(())
}
