mod chat;
mod send;

use std::sync::Arc;

use anyhow::Context;
use futures_util::StreamExt;

use crate::{data::Data, error::BotResult, services::redis};

pub async fn receive_webhook(data: Arc<Data>) -> BotResult {
	let mut sub = redis::redis_psub(data.clone(), "ermc:webhook").await?;

	println!("Webhook recv subscribed");

	let mut stream = sub.on_message();

	while let Some(msg) = stream.next().await {
		let payload: String = msg
			.get_payload()
			.context("Failed to get payload")?;

		println!("RECEIVED: {}", payload);

		let channel = msg.get_channel_name();
		let prefix = "ermc:webhook";

		let subchannel = channel
			.strip_prefix(prefix)
			.unwrap();

		match subchannel {
			"chat" => chat::chat_webhook(data.clone(), &payload).await?,
			"sleep" => (),
			"death" => (),
			"adv" => (),
			_ => continue,
		};
	}

	Ok(())
}
