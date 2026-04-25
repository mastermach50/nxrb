use std::process::{Command, ExitStatus};
use colored::{Colorize, ColoredString};
use tabled::{Table, Tabled, settings::Style};
use anyhow::{Context, Result};
use std::time::Duration;

pub fn execute_cmd(cmd: Vec<&str>) -> Result<ExitStatus> {
    let status = Command::new(cmd[0])
        .args(cmd.iter().skip(1))
        .status()
        .context(format!("Failed to execute {}", cmd[0]))?;

    Ok(status)
}

#[derive(Tabled)]
struct BuildStatus {
    status: ColoredString,
    message: ColoredString,
    time: ColoredString
}

pub fn print_build_status(status: ColoredString, message: ColoredString, time: Duration) {
    let mut build_status_table = Table::kv(vec![BuildStatus {
        status: status,
        message: message,
        time: humantime::format_duration(time).to_string().blue()
    }]);
    println!("{}", build_status_table.with(Style::rounded().remove_horizontals()));
}