use std::sync::Arc;

use poise::serenity_prelude::colours::branding::WHITE;
use serde::Deserialize;

use crate::{
	context::TaskCtx,
	error::BotResult,
	tasks::webhook::send::{self, Embed, Payload},
};

#[derive(Deserialize)]
struct Death {
	ign: String,
	uuid: String,
	msg: String,
}

impl From<Death> for Payload {
	fn from(death: Death) -> Self {
		let embed = Embed {
			title: format!("💀 {}", death.msg),
			color: WHITE.0,
		};

		Self {
			username: death.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", death.uuid),
			content: None,
			embeds: Some(vec![embed]),
		}
	}
}

pub async fn death_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(death) = serde_json::from_str::<Death>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(death);

	send::send_webhook(ctx.data.clone(), &webhook).await?;

	Ok(())
}
