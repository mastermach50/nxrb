use std::fs;
use anyhow::{Context, Ok, Result};
use toml;
use whoami;
use rand::{self, distr::SampleString};
use serde::{Serialize, Deserialize};

static CONFIG_FILE: &str = ".nxrb.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub dbus: DBus,
    pub git: Git,
    pub ntfy: Ntfy
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DBus {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Git {
    pub username: String,
    pub email: String,
    pub repo: String,
    pub branch: String,
    pub commit_on_success: bool,
    pub push_on_success: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ntfy {
    pub username: String,
    pub server: String,
    pub channel: String,
    pub token: String,
    pub icon: String
}

pub fn get_config() -> Result<Config> {
    let result = fs::read_to_string(CONFIG_FILE);
    if result.is_ok() {
        let config: Config = toml::from_str(&result.unwrap())?;
        return Ok(config);
    } else {
        let default_config = write_and_get_default_config()?;
        return Ok(default_config);
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