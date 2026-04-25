use std::{env, os::unix::process::CommandExt};
use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
use log::debug;
use std::process::Command;
use colored::Colorize;

mod cli;
mod config;
mod notification;

use notification::exit_sequence;

fn main() -> Result<()> {

    env_logger::init();
    let args = cli::Args::parse();

    // Elevate to root if not already running as root
    elevate_if_needed().context("Failed to elevate to root")?;

    println!("{}", "Running as root".green());

    debug!("Switch: {}", args.switch);
    debug!("Boot: {}", args.boot);
    debug!("Test: {}", args.test);
    debug!("Notify: {}", args.notify);
    debug!("Message: {}", args.message.clone().unwrap_or("<Not Provided>".to_string()));

    let config = config::get_config()
        .context("Could not load config")?;

    debug!("{:?}", config);

    if args.update {
        let flake_update_cmd = ["nix", "flake", "update", "-vv"];

        println!("-- {} [{}]", "Updating flake".yellow(), flake_update_cmd.join(" "));

        let status = Command::new(flake_update_cmd[0])
            .args(flake_update_cmd.iter().skip(1))
            .status()
            .context(format!("Failed to execute [{}]", flake_update_cmd.join(" ")))?;
        if !status.success() {
            println!("-- {}", "Failed to update flake".red());
            exit_sequence("Failed to update flake".to_string(), args, config);
        }
    }

    anyhow::Ok(())
}

fn elevate_if_needed() -> Result<()> {
    if unsafe { libc::getuid() != 0 } {
        println!("{}", "Not running as root, elevation required".yellow());

        let current_exe = env::current_exe().context("Failed to get current exe")?;
        let args: Vec<String> = env::args().skip(1).collect();
        let error = Command::new("sudo")
            .arg(current_exe)
            .args(args)
            .exec();

        println!("{}: {}", "Failed to elevate to root".red(), error);
        anyhow::bail!("Failed to elevate to root")
    } else {
        Ok(())
    }
}