use std::{env, os::unix::process::CommandExt};
use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
// use log::debug;
use std::process::Command;
use colored::Colorize;

mod cli;
mod config;
mod notification;

use notification::exit_sequence;

use crate::notification::{send_dbus_notification, send_ntfy_notification};

fn main() -> Result<()> {

    env_logger::init();
    let args = cli::Args::parse();

    // Elevate to root if not already running as root
    elevate_if_needed().context("Failed to elevate to root")?;

    println!("{}", "Running as root".green());

    // debug!("Switch: {}", args.switch);
    // debug!("Boot: {}", args.boot);
    // debug!("Test: {}", args.test);
    // debug!("Notify: {}", args.notify);
    // debug!("Message: {}", args.message.clone().unwrap_or("<Not Provided>".to_string()));

    let config = config::get_config()
        .context("Could not load config")?;

    // debug!("{:?}", config);

    let start_time = std::time::Instant::now();

    // Flake update
    if args.update {
        let flake_update_cmd = vec!["nix", "flake", "update", "-vv"];

        println!("-- {} [{}]", "Updating flake".yellow(), flake_update_cmd.join(" "));

        let output = Command::new(flake_update_cmd[0])
            .args(flake_update_cmd.iter().skip(1))
            .output()
            .context("Failed to execute nix")?;
        if output.status.success() {
            println!("-- {}", "Flake updated successfully".green());
        } else {
            eprintln!("-- {}", "Failed to update flake".red());
            exit_sequence("Failed to update flake", args.clone(), config.clone());
        }
    }

    // NixOS rebuild
    let build_mode = args.clone().get_build_mode();
    let rebuild_cmd = vec!["nixos-rebuild", build_mode];
    println!("-- {} [{}]", "Starting NixOS build".yellow(), rebuild_cmd.join(" "));
    let output =Command::new(rebuild_cmd[0])
        .args(rebuild_cmd.iter().skip(1))
        .output()
        .context("Failed to execute nixos-rebuild")?;
    if output.status.success() {
        println!("-- {}", "NixOS rebuilt successfully");
    } else {
        eprintln!("-- {}", "Failed to rebuild system".red());
        exit_sequence("Failed to rebuild system", args.clone(), config.clone());
    }

    // Git commit
    if config.git.commit_on_success {
        println!("-- {}", "Committing files".yellow());
        let mut commit_msg = vec![format!("[{}]", build_mode)];
        if let Some(ref msg) = args.message {
            commit_msg.insert(1, msg.clone());
        }
        let status = Command::new("git")
            .args(["checkout", "-b", &config.git.branch])
            .status()
            .context("Failed to execute git")?;
        if !status.success() {
            eprintln!("-- {} {}", "Failed to switch to".yellow(), config.git.branch);
            exit_sequence("Failed to switch to branch", args.clone(), config.clone());
        }
        let output = Command::new("git")
            .args([
                "commit",
                "-m",
                &commit_msg.join(" "),
                &format!("--author=\"{} <{}>\"", config.git.username, config.git.email)
            ])
            .output()
            .context("Failed to execute git")?;
        if output.status.success() {
            println!("-- {}", "Files committed successfully".green());
        } else {
            eprintln!("-- {}", "Failed to commit files".red());
            exit_sequence("Failed to commit files", args.clone(), config.clone());
        }
    }

    // Git push
    if config.git.push_on_success {
        println!("-- {}", "Pushing changes".yellow());
        let output = Command::new("git")
            .args(["push", "--set-upstream", "origin", &config.git.branch])
            .output()
            .context("Failed to execute git")?;
        if output.status.success() {
            println!("-- {}", "Changes pushed successfully");
        } else {
            eprintln!("-- {}", "Failed to push changes");
            exit_sequence("Failed to push changes", args.clone(), config.clone());
        }
    }

    // Finishing notifications
    let message_title = "✅ NixOS rebuild completed successfully";
    let message_body = format!("{}\nFinished build in {}sec", config.git.branch, humantime::format_duration(start_time.elapsed()));
    send_dbus_notification(config.clone(), message_title, &message_body);
    if args.notify {
        send_ntfy_notification(config.clone(), message_title, &message_body);
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