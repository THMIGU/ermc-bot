use std::fs;

use anyhow::Context;
use serde::Deserialize;

use crate::error::BotResult;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
	pub discord: DiscordConfig,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
	pub token: String,
	pub owner_id: u64,
	pub essress_id: u64,
}

impl Config {
	pub fn load() -> BotResult<Self> {
		let config_str =
			fs::read_to_string(CONFIG_PATH).context(format!("Failed to read {}", CONFIG_PATH))?;
		let config: Config =
			toml::from_str(&config_str).context(format!("Failed to parse {}", CONFIG_PATH))?;

		Ok(config)
	}
}
