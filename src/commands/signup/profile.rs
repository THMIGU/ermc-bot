use anyhow::{Context, Ok};
use reqwest::Client;
use serde::Deserialize;

use crate::error::BotResult;

const ENDPOINT: &str = "https://api.mojang.com/users/profiles/minecraft";

#[derive(Deserialize)]
pub struct Profile {
	#[serde(rename = "id")]
	pub uuid: String,
	pub name: String,
}

pub async fn get_profile(client: &Client, ign: &str) -> BotResult<Option<Profile>> {
	let response = client
		.get(format!("{}/{}", ENDPOINT, ign))
		.send()
		.await
		.context("Profile request failed")?;

	if !response.status().is_success() {
		return Ok(None);
	}

	let profile: Profile = response
		.json()
		.await
		.context("Failed to parse response")?;

	Ok(Some(profile))
}
