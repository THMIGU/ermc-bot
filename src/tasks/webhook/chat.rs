use serde::Deserialize;

use crate::{context::Ctx, error::BotResult};

#[derive(Deserialize)]
struct Chat {
	ign: String,
	uuid: String,
	msg: String,
}

pub async fn chat_webhook(ctx: Ctx<'_>, payload: &str) -> BotResult {
	let Ok(chat) = serde_json::from_str::<Chat>(payload) else {
		return Ok(());
	};

	Ok(())
}
