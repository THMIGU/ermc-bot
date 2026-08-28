use std::sync::Arc;

use poise::serenity_prelude::{CreateEmbed, ExecuteWebhook, colours::branding::WHITE};
use serde::Deserialize;

use crate::{context::TaskCtx, error::BotResult, tasks::webhook::send};

#[derive(Deserialize)]
struct Advancement {
	ign: String,
	uuid: String,
	name: String,
}

pub async fn adv_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(adv) = serde_json::from_str::<Advancement>(payload) else {
		return Ok(());
	};

	let avatar_url = format!("https://visage.surgeplay.com/head/512/{}", adv.uuid);
	let title = format!("✨ {} has achieved `[{}]`", adv.ign, adv.name);

	let embed = CreateEmbed::new()
		.title(title)
		.color(WHITE);

	let builder = ExecuteWebhook::new()
		.username(adv.ign)
		.avatar_url(avatar_url)
		.embed(embed);

	send::send_webhook(ctx, builder).await?;

	Ok(())
}
