use std::sync::Arc;

use poise::serenity_prelude::ExecuteWebhook;
use serde::Deserialize;

use crate::{context::TaskCtx, error::BotResult, tasks::webhook::send};

#[derive(Deserialize)]
struct Chat {
	ign: String,
	uuid: String,
	msg: String,
}

pub async fn chat_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(chat) = serde_json::from_str::<Chat>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", chat.uuid);
	let builder = ExecuteWebhook::new()
		.username(chat.ign)
		.avatar_url(avatar_url)
		.content(chat.msg);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}
