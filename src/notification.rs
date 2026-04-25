use crate::cli;
use crate::config;
use colored::Colorize;
use reqwest::blocking::Client;

pub fn exit_sequence(error: &str, args: cli::Args, config: config::Config) {
    let message_title = "❌ Failed to build NixOS";
    let message_body = format!("{}\n{}", config.git.branch, error);

    send_dbus_notification(config.clone(), message_title, &message_body);

    if args.notify {
        send_ntfy_notification(config, message_title, &message_body);
    }

    std::process::exit(-1);
}

pub fn send_dbus_notification(config: config::Config, title: &str, body: &str) {}

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
                println!("{}", "Ntfy returned error".yellow())
            }
        },
        Err(err) => println!("Notification errored: {}", err)
    };
}