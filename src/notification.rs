use crate::cli;
use crate::config;
use reqwest::blocking::Client;

pub fn exit_sequence(error: String, args: cli::Args, config: config::Config) {
    let message_title = "❌ Failed to build NixOS";
    let message_body = format!("{}\n{}", config.git.branch, error);

    if args.notify {
        let client = Client::new();
        let res = client.post(
            format!("{}/{}", config.ntfy.server, config.ntfy.channel))
            .bearer_auth(config.ntfy.token)
            .header("Title", message_title)
            .header("icon", config.ntfy.icon)
            .body(message_body)
            .send();

        match res {
            Ok(response) => println!("{:?}", response),
            Err(err) => println!("Notification errored: {}", err)
        };
    }

    std::process::exit(-1);
}