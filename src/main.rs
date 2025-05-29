// src/main.rs

use colored::Colorize;
use std::io;
use clap::{Parser, Subcommand};  // ต้อง use Parser และ Subcommand มาใช้

// module
mod commands;
mod echo_test;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ls, //run the ls -l command.
    Neofetch, //run the neofetch command.
    RunScript {
        // The path to the script to run.
        #[arg(short,long, default_value = "./my_script.sh")]
        script_path: String,
        // An argument to pass to the script.
        #[arg(short, long, default_value = "--hello-from-rust")]
        arg: String,
    },
    //custom command
    Exec {
        #[arg(long)]
        cmd: String,
    },
    /// Gets and processes output from 'ls -l' (counts lines).
    CountLsOutput,
    /// Runs 'echo I use Ubuntu btw'.
    Sus,
    /// Runs 'echo Sun'.
    EchoSun,
    /// Runs 'echo Wow'.
    EchoWow,
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

fn main() -> Result<(),io::Error>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls => commands::run_shell_command("ls", &["-l"])?,
        Commands::Neofetch => commands::run_shell_command("neofetch", &str_nil())?,
        Commands::RunScript { script_path, arg } => commands::run_shell_command(script_path, &[arg])?,
        Commands::Exec { cmd } => exec(cmd)?,
        Commands::CountLsOutput => count_ls_output(),
        Commands::Sus => commands::run_shell_command("echo", &["I use Ubuntu btw"])?,
        Commands::EchoSun => echo_test::echo_sun()?,
        Commands::EchoWow => echo_test::echo_wow()?,
    }
    Ok(())
}
