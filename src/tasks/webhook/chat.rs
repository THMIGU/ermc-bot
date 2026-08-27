use std::sync::Arc;

use serde::Deserialize;

use crate::{
	data::Data,
	error::BotResult,
	tasks::webhook::send::{self, Payload},
};

#[derive(Deserialize)]
struct Chat {
	ign: String,
	uuid: String,
	msg: String,
}

impl From<Chat> for Payload {
	fn from(chat: Chat) -> Self {
		Self {
			username: chat.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", chat.uuid),
			content: Some(chat.msg),
			embeds: None,
		}
	}
}

pub async fn chat_webhook(data: Arc<Data>, payload: &str) -> BotResult {
	let Ok(chat) = serde_json::from_str::<Chat>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(chat);

	send::send_webhook(data, &webhook).await?;

	Ok(())
}
