use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use env_logger;
use std::process::Command;
use std::{env, os::unix::process::CommandExt};

mod cli;
mod config;
mod helpers;
mod notification;

use helpers::execute_cmd;
use notification::{exit_sequence, send_dbus_notification, send_ntfy_notification};

fn main() -> Result<()> {
    env_logger::init();
    let args = cli::Args::parse();

    // Elevate to root if not already running as root
    elevate_if_needed()?;

    println!("-- {}", "Running as root".green());

    let config = config::get_config().context("Could not load config")?;

    let start_time = std::time::Instant::now();

    // Flake update
    if args.update {
        let flake_update_cmd = vec!["nix", "flake", "update", "-vv"];
        println!(
            "-- {} [{}]",
            "Updating flake".yellow(),
            flake_update_cmd.join(" ")
        );
        let status = execute_cmd(flake_update_cmd)?;
        if status.success() {
            println!("-- {}", "Flake updated successfully".green());
        } else {
            exit_sequence("Failed to update flake", args.clone(), config.clone())?;
        }
    }

    // NixOS rebuild
    let build_mode = args.clone().get_build_mode();
    let rebuild_cmd = vec!["nixos-rebuild", build_mode];
    println!(
        "-- {} [{}]",
        "Starting NixOS build".yellow(),
        rebuild_cmd.join(" ")
    );
    let status = execute_cmd(rebuild_cmd)?;
    if status.success() {
        println!("-- {}", "NixOS rebuilt successfully");
    } else {
        exit_sequence("Failed to rebuild system", args.clone(), config.clone())?;
    }

    // Git commit
    if config.git.commit_on_success {
        println!("-- {}", "Committing files".yellow());
        let mut commit_msg = vec![format!("[{}]", build_mode)];
        if let Some(ref msg) = args.message {
            commit_msg.insert(1, msg.clone());
        }
        let status = execute_cmd(vec!["git", "checkout", "-b", &config.git.branch])?;
        if !status.success() {
            exit_sequence("Failed to switch to branch", args.clone(), config.clone())?;
        }
        let status = execute_cmd(vec![
            "git",
            "commit",
            "-m",
            &commit_msg.join(" "),
            &format!(
                "--author=\"{} <{}>\"",
                config.git.username, config.git.email
            ),
        ])?;
        if status.success() {
            println!("-- {}", "Files committed successfully".green());
        } else {
            exit_sequence("Failed to commit files", args.clone(), config.clone())?;
        }
    }

    // Git push
    if config.git.push_on_success {
        println!("-- {}", "Pushing changes".yellow());
        let status = execute_cmd(vec![
            "git",
            "push",
            "--set-upstream",
            "origin",
            &config.git.branch,
        ])?;
        if status.success() {
            println!("-- {}", "Changes pushed successfully");
        } else {
            exit_sequence("Failed to push changes", args.clone(), config.clone())?;
        }
    }

    // Finishing notifications
    let message_title = "✅ NixOS rebuild completed successfully";
    let message_body = format!(
        "{}\nFinished build in {}sec",
        config.git.branch,
        humantime::format_duration(start_time.elapsed())
    );
    send_dbus_notification(config.clone(), message_title, &message_body)?;
    if args.notify {
        send_ntfy_notification(config.clone(), message_title, &message_body);
    }

    anyhow::Ok(())
}

fn elevate_if_needed() -> Result<()> {
    if unsafe { libc::getuid() != 0 } {
        println!("-- {}", "Not running as root, trying to elevate".yellow());

        let current_exe = env::current_exe().context("Failed to get current exe")?;
        let args: Vec<String> = env::args().skip(1).collect();
        let error = Command::new("sudo").arg(current_exe).args(args).exec();

        eprintln!("-- {}: {}", "Failed to elevate to root".red(), error);
        anyhow::bail!("Failed to elevate to root")
    } else {
        Ok(())
    }
}
