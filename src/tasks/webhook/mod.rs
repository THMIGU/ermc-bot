mod chat;
mod send;

use anyhow::Context;
use futures_util::StreamExt;

use crate::{context::Ctx, error::BotResult, services::redis};

pub async fn receive_webhook(ctx: Ctx<'_>) -> BotResult {
	let mut sub = redis::redis_psub(ctx, "ermc:webhook").await?;
	let mut stream = sub.on_message();

	while let Some(msg) = stream.next().await {
		let payload: String = msg
			.get_payload()
			.context("Failed to get payload")?;

		let channel = msg.get_channel_name();
		let prefix = "ermc:webhook";

		let subchannel = channel
			.strip_prefix(prefix)
			.unwrap();

		match subchannel {
			"chat" => chat::chat_webhook(ctx, &payload).await?,
			"sleep" => (),
			"death" => (),
			"adv" => (),
			_ => continue,
		};
	}

	Ok(())
}
