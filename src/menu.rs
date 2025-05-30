//add menu.rs

use std::io;
use clap::{Parser, Subcommand};
use crate::commands;
use crate::echo_test;
use crate::utils;
use crate::utils::{
    exec,count_ls_output,str_nil
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs the 'ls -l' command
    Ls,
    /// Runs the neofect conmmand
    Neofetch,
    /// Runs the 'neofetch' command
    RunScript {
        // The path to the script to run.
        #[arg(short,long, default_value = "./my_script.sh")]
        script_path: String,
        // An argument to pass to the script.
        #[arg(short, long, default_value = "--hello-from-rust")]
        arg: String,
    },
    /// Executes a custom shell command
    /// Example: `cargo run -- exec --cmd "grep -i rust Cargo.toml"`
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
    /// Shows the current configuration.
    ShowConfig, // <<< เพิ่ม Subcommand ใหม่
    /// Initializes a default config file.
    InitConfig, // <<< เพิ่ม Subcommand ใหม่
}


pub fn menu() -> Result<(), io::Error>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls => utils::ls()?,
        Commands::Neofetch => commands::run_shell_command("neofetch", &str_nil())?,
        Commands::RunScript { script_path, arg } => commands::run_shell_command(script_path, &[arg])?,
        Commands::Exec { cmd } => exec(cmd)?,
        Commands::CountLsOutput => count_ls_output(),
        Commands::Sus => utils::sus()?,
        Commands::EchoSun => echo_test::echo_sun()?,
        Commands::EchoWow => echo_test::echo_wow()?,
        Commands::ShowConfig => utils::show_config(),
        Commands::InitConfig => utils::init_config(),
    }
    Ok(())
}