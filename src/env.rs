use anyhow::{bail, Error, Result};
use envy::Error as EnvyError;
use serde::de::Error as _;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_debug_mode")]
    pub debug_mode: bool, // export DEBUG_MODE="true"
    #[serde(default = "default_port")]
    pub noticube_port: u16, // export NOTICUBE_PORT="2525"
    #[serde(default = "default_allow_ip")]
    pub noticube_ip: String, // export NOTICUBE_IP="0.0.0.0"
    pub slack_bot_token: String,  // export SLACK_BOT_TOKEN="xxxxxx"
    pub slack_channel_id: String, // export SLACK_CHANNEL_ID="yyyyyy"
}

fn default_debug_mode() -> bool {
    false
}

fn default_port() -> u16 {
    2525
}

fn default_allow_ip() -> String {
    "127.0.0.1".to_string()
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let envs = envy::from_env::<Config>().map_err(Error::new)?;
        if envs.slack_bot_token.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: SLACK_BOT_TOKEN"
            ))
        }
        if envs.slack_channel_id.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: SLACK_CHANNEL_ID"
            ))
        }
        Ok(envs)
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.noticube_ip, self.noticube_port)
    }
}
