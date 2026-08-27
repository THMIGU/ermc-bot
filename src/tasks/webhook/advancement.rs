use std::sync::Arc;

use poise::serenity_prelude::colours::branding::WHITE;
use serde::Deserialize;

use crate::{
	context::TaskCtx,
	error::BotResult,
	tasks::webhook::send::{self, Embed, Payload},
};

#[derive(Deserialize)]
struct Advancement {
	ign: String,
	uuid: String,
	name: String,
}

impl From<Advancement> for Payload {
	fn from(adv: Advancement) -> Self {
		let embed = Embed {
			title: format!("✨ {} has achieved `[{}]`", adv.ign, adv.name),
			color: WHITE.0,
		};

		Self {
			username: adv.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", adv.uuid),
			content: None,
			embeds: Some(vec![embed]),
		}
	}
}

pub async fn adv_webhook(ctx: Arc<TaskCtx>, payload: &str) -> BotResult {
	let Ok(death) = serde_json::from_str::<Advancement>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(death);

	send::send_webhook(ctx.data.clone(), &webhook).await?;

	Ok(())
}
