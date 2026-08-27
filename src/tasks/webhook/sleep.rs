use std::sync::Arc;

use poise::serenity_prelude::colours::branding::WHITE;
use serde::Deserialize;

use crate::{
	context::TaskCtx,
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
			color: WHITE.0,
		};

		Self {
			username: sleep.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", sleep.uuid),
			content: None,
			embeds: Some(vec![embed]),
		}
	}
}

pub async fn sleep_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(sleep) = serde_json::from_str::<Sleep>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(sleep);

	send::send_webhook(ctx.data.clone(), &webhook).await?;

	Ok(())
}
