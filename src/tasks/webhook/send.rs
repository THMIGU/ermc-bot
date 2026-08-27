use std::sync::Arc;

use serde::Serialize;

use crate::{data::Data, error::BotResult};

#[derive(Serialize)]
pub struct Payload {
	pub username: String,
	pub avatar_url: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub embeds: Option<Vec<Embed>>,
}

#[derive(Serialize)]
pub struct Embed {
	pub title: String,
	pub color: u32,
}

pub async fn send_webhook(data: Arc<Data>, payload: &Payload) -> BotResult {
	let client = &data.http_client;
	let webhook_url = &data
		.config
		.discord
		.webhook_url;

	client
		.post(webhook_url)
		.json(payload)
		.send()
		.await?
		.error_for_status()?;

	Ok(())
}
