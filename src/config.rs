use std::{fs, io};
use anyhow::{Context, Result};
use toml;
use whoami;
use rand::{self, distr::SampleString};
use serde::{Serialize, Deserialize};

static CONFIG_FILE: &str = ".nxrb.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub dbus: DBus,
    pub git: Git,
    pub ntfy: Ntfy
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBus {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Git {
    pub username: String,
    pub email: String,
    pub repo: String,
    pub branch: String,
    pub commit_on_success: bool,
    pub push_on_success: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ntfy {
    pub username: String,
    pub server: String,
    pub channel: String,
    pub token: String,
    pub icon: String
}

pub fn get_config() -> Result<Config> {
    match fs::read_to_string(CONFIG_FILE) {
        Ok(result) => {
            let config: Config = toml::from_str(&result)?;
            return Ok(config);
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("Config file not found, creating file at {CONFIG_FILE}");
            let default_config = write_and_get_default_config()?;
            return Ok(default_config);
        }
        Err(e) => {Err(anyhow::anyhow!(e))}
    }
}

fn write_and_get_default_config() -> Result<Config> {

    let default_config_str = format!(r#"[dbus]
# the user who should receive dbus notifications
username = "{username}"

[git]
# git username and email that should be used for committing
username = "{username}"
email = "{username}@{hostname}"
repo = "somerepo"
branch = "somebranch" # different branches can be used for different devices
commit_on_success = true
push_on_success = true

[ntfy]
# ntfy server details for notifications
username = "{username}"
server = "https://ntfy.sh"
channel = "{random_alpha}"
token = "tk_thisisnotarealtoken"
icon = "https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-colours.svg""#,
    username = whoami::username().context("Failed to get username")?,
    hostname = whoami::hostname().context("Failed to get hostname")?,
    random_alpha = rand::distr::Alphabetic.sample_string(&mut rand::rng(), 8)
    );

    let default_config = toml::from_str(&default_config_str)
        .expect("Failed to parse default config");

    fs::write(CONFIG_FILE, default_config_str)
        .context("Failed to write default config")?;

    Ok(default_config)
}