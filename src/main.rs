// src/main.rs
use std::io;
use menu::menu;

// module
mod commands;
mod echo_test;
mod utils;
mod menu;
mod config;

fn main() -> Result<(),io::Error>{
    menu()
}
