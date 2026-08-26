use anyhow::Context;
use redis::{AsyncTypedCommands, aio::PubSub};

use crate::{context::Ctx, error::BotResult};

pub async fn redis_sub(ctx: Ctx<'_>, channel: &str) -> BotResult<PubSub> {
	let client = &ctx.data().redis_client;

	let mut pubsub = client
		.get_async_pubsub()
		.await
		.context("Failed to get async pubsub")?;

	println!("Connected to Redis");

	pubsub
		.subscribe(channel)
		.await
		.context("Failed to subscribe to channel")?;

	Ok(pubsub)
}

pub async fn redis_pub(ctx: Ctx<'_>, channel: &str, message: &str) -> BotResult {
	let mut manager = ctx
		.data()
		.redis_manager
		.clone();

	manager
		.publish(channel, message)
		.await
		.context("Failed to publish message")?;

	Ok(())
}
