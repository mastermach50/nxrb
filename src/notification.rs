use crate::cli;
use crate::config;
use std::process::Command;
use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::blocking::Client;

pub fn exit_sequence(error: &str, args: cli::Args, config: config::Config) -> Result<()> {
    let message_title = "❌ Failed to build NixOS";
    let message_body = format!("{}\n{}", config.git.branch, error);

    send_dbus_notification(config.clone(), message_title, &message_body)?;

    if args.notify {
        send_ntfy_notification(config, message_title, &message_body);
    }

    std::process::exit(-1);
}

pub fn send_dbus_notification(config: config::Config, title: &str, body: &str) -> Result<()> {
    let output = Command::new("id")
        .args(["-u", &config.dbus.username])
        .output()
        .context("Failed to execute id")?;
    if !output.stderr.is_empty() {
        return Err(anyhow::anyhow!("Failed to get uid from username"));
    } else {
        let output = Command::new("notify-send")
            .env("DBUS_SESSION_BUS_ADDRESS", &format!("unix:path=/run/user/{}/bus", String::from_utf8(output.stdout).unwrap()))
            .args([title, body])
            .output()
            .context("Failed to execute notify-send")?;
        if output.status.success() {
            println!("-- {}", "DBUS notification sent".green());
        }
    }

    Ok(())
}

pub fn send_ntfy_notification(config: config::Config, title: &str, body: &str ) {
    let client = Client::new();
    let res = client.post(
        format!("{}/{}", config.ntfy.server, config.ntfy.channel))
        .bearer_auth(config.ntfy.token)
        .header("Title", title.to_string())
        .header("icon", config.ntfy.icon)
        .body(body.to_string())
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("{}", "Ntfy notification posted".green())
            } else {
                eprintln!("{}", "Ntfy returned error".yellow())
            }
        },
        Err(err) => println!("Notification errored: {}", err)
    };
}