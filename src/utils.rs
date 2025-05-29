// src/utils.rs

use std::io::{
        stdin, stdout, Write
};

pub fn _input_string(prompt: &str) -> String {
    print!("{}",prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}