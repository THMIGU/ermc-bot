use std::sync::Arc;

use poise::serenity_prelude::{CreateEmbed, ExecuteWebhook, colours::branding::WHITE};
use serde::Deserialize;

use crate::{context::TaskCtx, error::BotResult, tasks::webhook::send};

#[derive(Deserialize)]
struct Death {
	ign: String,
	uuid: String,
	msg: String,
}

pub async fn death_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(death) = serde_json::from_str::<Death>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", death.uuid);
	let title = format!("💀 {}", death.msg);

	let embed = CreateEmbed::new()
		.title(title)
		.color(WHITE);

	let builder = ExecuteWebhook::new()
		.username(death.ign)
		.avatar_url(avatar_url)
		.embed(embed);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}
