// src/utils.rs

use colored::Colorize;
use std::io::{
        self,stdin, stdout, Write
};
use crate::commands;
use crate::config;

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

pub fn ls() -> Result<(),std::io::Error> {    
    let args_str: Vec<&'static str> = config::default_ls_args_str();
    commands::run_shell_command("ls", &args_str)?;
    Ok(())
}

pub fn sus() -> Result<(), io::Error> {
    let binding: String = config::default_alias_sus();
    let parts: Vec<&str> = binding.split_whitespace().collect();
    if let Some((command,args)) = parts.split_first(){
        return commands::run_shell_command(command,args)
    } else {
        eprintln!("{}", "Error: Alias sus config is empty or invalid.".red());
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid alias sus config"));
    }
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

pub fn show_config() {
    println!("\n--- Current Configuration ---");
    println!("{:#?}", config::Config::load_and_create_default("../config.toml"));
}

pub fn init_config() {
    let default_config = config::Config::default();
    if let Err(e) = default_config.save("config.toml") {
        eprintln!("Error saving default config: {}", e);
    }
    println!("Default config saved to config.toml");
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