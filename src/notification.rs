use crate::config;
use std::process::Command;
use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::blocking::Client;

pub fn send_dbus_notification(config: config::Config, title: &str, body: &str) -> Result<()> {
    // TODO fix dbus notifications
    // let output = Command::new("id")
    //     .args(["-u", &config.dbus.username])
    //     .output()
    //     .context("Failed to execute id")?;
    // if !output.stderr.is_empty() {
    //     return Err(anyhow::anyhow!("Failed to get uid from username"));
    // } else {
    //     let bus = format!("unix:path=/run/user/{}/bus", String::from_utf8(output.stdout)?.trim());
    //     let output = Command::new("notify-send")
    //         .env("DBUS_SESSION_BUS_ADDRESS", &bus)
    //         .args([title, body])
    //         .output()
    //         .context("Failed to execute notify-send")?;
    //     if output.status.success() {
    //         println!("-- {}", "DBUS notification sent".green());
    //     } else {
    //         eprintln!("-- {}: {}", "Failed to send dbus notification", String::from_utf8(output.stderr).unwrap());
    //     }
    // }

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