use std::collections::HashMap;

use anyhow::{Context, Ok};
use reqwest::Client;

use crate::error::BotResult;

const ENDPOINT: &str = "https://api.mojang.com/users/profiles/minecraft";

pub async fn get_uuid(client: &Client, ign: &str) -> BotResult<Option<String>> {
	let response = client
		.get(format!("{}/{}", ENDPOINT, ign))
		.send()
		.await
		.context("Failed to get UUID")?;

	if !response.status().is_success() {
		return Ok(None);
	}

	let map: HashMap<String, String> = response
		.json()
		.await
		.context("Failed to parse response")?;

	let uuid = map
		.get("id")
		.context("Failed to find UUID in response")?;

	Ok(Some(uuid.clone()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_get_uuid() {
		let client = Client::new();

		let uuid = get_uuid(&client, "THMIGU")
			.await
			.unwrap()
			.unwrap();

		println!("{uuid}");
	}
}
