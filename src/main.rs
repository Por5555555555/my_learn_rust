// src/main.rs
use std::io;
use menu::menu;

// module
mod commands;
mod echo_test;
mod utils;
mod menu;

fn main() -> Result<(),io::Error>{
    menu()
}
