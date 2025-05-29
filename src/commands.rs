// src/commands.rs

use colored::Colorize;
use std::io;
use std::process::Command;

pub fn run_shell_command(command_name: &str, args: &[&str]) -> Result<(), io::Error> {
    println!("\n--- Executing: {} {} ---", command_name.blue(), args.join(" ").blue());

    let output = Command::new(command_name)
         .args(args)
         .output()?;

    match output.status.success() {
        true => {
            println!("{}", "Command succeeded!".green());
            if !output.stdout.is_empty() {
                println!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            }
        },
        false => {
            println!("{}", "Command failed!".red());
            if !output.stderr.is_empty() {
                eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr).red());
            }
            eprintln!("Exit Status: {}", output.status.to_string().red());
        },
    }

    Ok(())
}

pub fn get_shell_command_output(command_name: &str, args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new(command_name)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let error_msg = format!(
            "Command failed: {} {} (Exit Status: {})\nStderr:\n{}",
            command_name,
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        Err(io::Error::new(io::ErrorKind::Other, error_msg))
    }
}

pub fn _ask_and_run_command_in_commands_module() -> Result<(), std::io::Error> {
    let user_command = crate::utils::_input_string("Enter a command to run (from commands module): ");

    if user_command.is_empty() {
        println!("No command entered in commands module.");
        return Ok(());
    }

    let parts: Vec<&str> = user_command.split_whitespace().collect();
    if let Some((cmd, args)) = parts.split_first() {
        run_shell_command(cmd, args)?;
    } else {
        println!("{}", "Invalid command format in commands module.".red());
    }
    Ok(())
}