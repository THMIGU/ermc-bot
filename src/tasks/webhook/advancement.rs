use std::sync::Arc;

use serde::Deserialize;

use crate::{
	data::Data,
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
			color: 0xFFFFFF,
		};

		Self {
			username: adv.ign,
			avatar_url: format!("https://visage.surgeplay.com/head/512/{}", adv.uuid),
			content: None,
			embeds: Some(vec![embed]),
		}
	}
}

pub async fn adv_webhook(data: Arc<Data>, payload: &str) -> BotResult {
	let Ok(death) = serde_json::from_str::<Advancement>(payload) else {
		return Ok(());
	};
	let webhook = Payload::from(death);

	send::send_webhook(data, &webhook).await?;

	Ok(())
}
