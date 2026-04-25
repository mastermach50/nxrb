use std::process::{Command, ExitStatus};
use tabled::{Table, settings::Style};
use anyhow::{Context, Result};
use std::time::Duration;

pub fn execute_cmd(cmd: Vec<&str>) -> Result<ExitStatus> {
    let status = Command::new(cmd[0])
        .args(cmd.iter().skip(1))
        .status()
        .context(format!("Failed to execute {}", cmd[0]))?;

    Ok(status)
}

pub fn print_build_status(status: &str, message: &str, time: Duration) {
    let build_status = [
        status,
        message,
        &humantime::format_duration(time).to_string(),
    ];
    let mut build_status_table = Table::new(build_status);
    println!("{}", build_status_table.with(Style::rounded()));
}