use std::sync::Arc;

use serde::Deserialize;

use crate::{
	data::Data,
	error::BotResult,
	tasks::webhook::send::{self, Embed, Payload},
};

#[derive(Deserialize)]
struct Sleep {
	ign: String,
	uuid: String,
}

impl From<Sleep> for Payload {
	fn from(sleep: Sleep) -> Self {
		let embed = Embed {
			title: format!("🌙 {} is sleeping", sleep.ign),
			color: 0xFFFFFF,
		};

		Self {
			username: sleep.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", sleep.uuid),
			content: None,
			embeds: Some(vec![embed]),
		}
	}
}

pub async fn sleep_webhook(data: Arc<Data>, payload: &str) -> BotResult {
	let Ok(sleep) = serde_json::from_str::<Sleep>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(sleep);

	send::send_webhook(data, &webhook).await?;

	Ok(())
}
