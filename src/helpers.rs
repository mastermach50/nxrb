use std::process::{Command, ExitStatus};

use anyhow::{Context, Result};

pub fn execute_cmd(cmd: Vec<&str>) -> Result<ExitStatus> {
    let status = Command::new(cmd[0])
        .args(cmd.iter().skip(1))
        .status()
        .context(format!("Failed to execute {}", cmd[0]))?;

    Ok(status)
}