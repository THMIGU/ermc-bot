use std::sync::Arc;

use poise::serenity_prelude::{CreateEmbed, ExecuteWebhook, colours::branding::WHITE};
use serde::Deserialize;

use crate::{context::TaskCtx, error::BotResult, tasks::webhook::send};

#[derive(Deserialize)]
struct Sleep {
	ign: String,
	uuid: String,
}

pub async fn sleep_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(sleep) = serde_json::from_str::<Sleep>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", sleep.uuid);
	let title = format!("💤 {} is sleeping", sleep.ign);

	let embed = CreateEmbed::new()
		.title(title)
		.color(WHITE);

	let builder = ExecuteWebhook::new()
		.username(sleep.ign)
		.avatar_url(avatar_url)
		.embed(embed);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}
