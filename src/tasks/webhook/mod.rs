mod advancement;
mod chat;
mod death;
mod send;
mod sleep;

use std::sync::Arc;

use anyhow::Context;
use futures_util::StreamExt;

use crate::{data::Data, error::BotResult, services::redis};

pub async fn receive_webhook(data: Arc<Data>) -> BotResult {
	let mut sub = redis::redis_psub(data.clone(), "ermc:webhook:*").await?;

	let mut stream = sub.on_message();

	while let Some(msg) = stream.next().await {
		let payload: String = msg
			.get_payload()
			.context("Failed to get payload")?;

		let channel = msg.get_channel_name();
		let prefix = "ermc:webhook:";

		let subchannel = channel
			.strip_prefix(prefix)
			.unwrap();

		match subchannel {
			"chat" => chat::chat_webhook(data.clone(), &payload).await?,
			"sleep" => sleep::sleep_webhook(data.clone(), &payload).await?,
			"death" => death::death_webhook(data.clone(), &payload).await?,
			"adv" => advancement::adv_webhook(data.clone(), &payload).await?,
			_ => continue,
		};
	}

	Ok(())
}
