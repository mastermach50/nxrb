use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use ctrlc;
use std::process::Command;
use std::time::Duration;
use std::{env, os::unix::process::CommandExt};

mod cli;
mod config;
mod helpers;
mod notification;

use helpers::execute_cmd;
use notification::{send_dbus_notification, send_ntfy_notification};

use crate::helpers::print_build_status;

fn main() -> Result<()> {
    // Parse cli args
    let args = cli::Args::parse();

    // Elevate to root if not already running as root
    elevate_if_needed()?;

    // Code from here on will only reach if the uid is 0 aka user is root
    println!("-- {}", "Running as root".green());

    // Parse config and start get current time to keep track of build time
    let config = config::get_config().context("Could not load config")?;
    let start_time = std::time::Instant::now();

    // Setup the interrupt handler
    let ctrlc_args = args.clone();
    let ctrlc_config = config.clone();
    ctrlc::set_handler(move || {
        fail_exit_sequence(
            "Encountered SIGINT",
            start_time.elapsed(),
            ctrlc_args.clone(),
            ctrlc_config.clone(),
        )
        .unwrap();
    })?;

    // Nix flake update
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
            fail_exit_sequence(
                "Failed to update flake",
                start_time.elapsed(),
                args.clone(),
                config.clone(),
            )?;
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
        fail_exit_sequence(
            "Failed to rebuild system",
            start_time.elapsed(),
            args.clone(),
            config.clone(),
        )?;
    }

    // Git commit
    if config.git.commit_on_success {
        println!("-- {}", "Committing files".yellow());
        let mut commit_msg = vec![format!("[{}]", build_mode)];
        if let Some(ref msg) = args.message {
            commit_msg.insert(1, msg.clone());
        }
        // Switch to correct branch
        let status = execute_cmd(vec!["git", "switch", "-C", &config.git.branch])?;
        if !status.success() {
            fail_exit_sequence(
                "Failed to switch to branch",
                start_time.elapsed(),
                args.clone(),
                config.clone(),
            )?;
        }
        // Stage files
        let status = execute_cmd(vec!["git", "add", "-A"])?;
        if !status.success() {
            fail_exit_sequence(
                "Failed to stage changes",
                start_time.elapsed(),
                args.clone(),
                config.clone(),
            )?;
        }
        // Commit
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
            fail_exit_sequence(
                "Failed to commit files",
                start_time.elapsed(),
                args.clone(),
                config.clone(),
            )?;
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
            fail_exit_sequence(
                "Failed to push changes",
                start_time.elapsed(),
                args.clone(),
                config.clone(),
            )?;
        }
    }

    // Finishing notifications
    let message_title = "✅ NixOS build completed successfully";
    let message_body = format!(
        "{}\nFinished build in {}sec",
        config.git.branch,
        humantime::format_duration(start_time.elapsed())
    );
    send_dbus_notification(config.clone(), message_title, &message_body)?;
    if args.notify {
        send_ntfy_notification(config.clone(), message_title, &message_body);
    }

    print_build_status(
        "SUCCESS".green(),
        "NixOS build completed successfully".green(),
        start_time.elapsed(),
    );

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

pub fn fail_exit_sequence(
    error: &str,
    time: Duration,
    args: cli::Args,
    config: config::Config,
) -> Result<()> {
    eprintln!("-- {}", error.red());

    let message_title = "❌ Failed to build NixOS";
    let message_body = format!(
        "{}\n{}\n{}",
        config.git.branch,
        error,
        humantime::format_duration(time)
    );

    send_dbus_notification(config.clone(), message_title, &message_body)?;
    if args.notify {
        send_ntfy_notification(config, message_title, &message_body);
    }

    print_build_status("ERROR".red(), error.red(), time);

    std::process::exit(-1);
}
