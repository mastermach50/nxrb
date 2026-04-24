use std::{fs, path::Path};

use anyhow::{Context, Ok, Result};

use toml;
use whoami;
use rand::{self, distr::SampleString};
use serde::{Serialize, Deserialize};

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
    pub commit_on_success: bool,
    pub push_on_success: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ntfy {
    pub username: String,
    pub server: String,
    pub channel: String,
    pub token: String
}

pub fn get_config() -> Result<Config> {
    let result = fs::read_to_string(".nxrbconf");
    if result.is_ok() {
        let config: Config = toml::from_str(&result.unwrap())?;
        return Ok(config);
    } else {
        let default_config = get_default_config()?;
        fs::write(".nxrbconf", toml::to_string_pretty(&default_config)?)?;
        return Ok(default_config);
    }

}

fn get_default_config() -> Result<Config> {

    let default_config = Config{
        dbus: DBus {
            username: whoami::username().context("Failed to get username")?
        },
        git: Git {
            username: whoami::username()?,
            email: format!("{}@{}", whoami::username()?, whoami::hostname().context("Failed to get hostname")?),
            commit_on_success: true,
            push_on_success: true
        },
        ntfy: Ntfy {
            username: whoami::username()?,
            server: "https://ntfy.sh".to_string(),
            channel: rand::distr::Alphabetic.sample_string(&mut rand::rng(), 8),
            token: format!("tk_thisisnotarealtoken")
        }
    };

    Ok(default_config)
}