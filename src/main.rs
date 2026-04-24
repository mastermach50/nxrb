use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod config;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    println!("Switch: {}", args.switch);
    println!("Boot: {}", args.boot);
    println!("Test: {}", args.test);
    println!("Notify: {}", args.notify);
    println!("Message: {}", args.message.unwrap_or("<Not Provided>".to_string()));

    let config = config::get_config()
        .context("Could not load config")?;

    println!("{:?}", config);

    anyhow::Ok(())
}
