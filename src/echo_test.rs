// src/echo_test.rs

use crate::commands; // เรียกใช้ module commands

pub fn echo_sun() -> Result<(), std::io::Error> {
    commands::run_shell_command("echo", &["Sun"])?;
    Ok(())
}

pub fn echo_wow() -> Result<(), std::io::Error> {
    crate::commands::run_shell_command("echo", &["wow"])?;
    Ok(())
}