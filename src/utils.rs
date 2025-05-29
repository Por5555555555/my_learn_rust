// src/utils.rs

use colored::Colorize;
use std::io::{
        self,stdin, stdout, Write
};
use crate::commands;

pub fn _input_string(prompt: &str) -> String {
    print!("{}",prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

pub fn str_nil() -> Vec<&'static str>{
    vec![""]
}

pub fn exec(cmd: &String) -> Result<(),io::Error> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if let Some((command, args)) = parts.split_first() {
        commands::run_shell_command(command, args)?;
    } else {
        eprintln!("{}", "Error: No command provided for --cmd.".red());
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No command provided"));
    }
    Ok(())
}

pub fn count_ls_output() {
    println!("\n{}","--- Processing `ls -l` output ---".purple());
    match commands::get_shell_command_output("ls", &["-l"]) {
        Ok(output_str) => {
            let lines: Vec<&str> = output_str.lines().collect();
            println!("Raw output:\n{}",output_str);
            println!("Number of lines in output: {}", lines.len().to_string().cyan());
        }
        Err(e) => eprintln!("{} Failed to get output: {}", "Error!".red(), e)
    }
}