use anyhow::Context;
use futures_util::StreamExt;
use redis::{AsyncTypedCommands, Client};

use crate::error::BotResult;

pub async fn redis_sub() -> BotResult {
	let client = Client::open("redis://10.0.0.144:52003/")?;

	let mut pubsub = client
		.get_async_pubsub()
		.await
		.context("Failed to get async pubsub")?;

	println!("Connected to Redis");

	pubsub
		.subscribe("cm:broadcast")
		.await
		.context("Failed to subscribe to channel")?;

	let mut stream = pubsub.on_message();
	while let Some(msg) = stream.next().await {
		let payload: String = msg
			.get_payload()
			.context("Failed to get payload")?;

		println!("{payload}");
	}

	Ok(())
}

pub async fn redis_pub() -> BotResult {
	let client = Client::open("redis://10.0.0.144:52003/")?;

	let mut con = client
		.get_multiplexed_async_connection()
		.await
		.context("Failed to get async con")?;

	println!("Connected to Redis");

	let _ = con
		.publish("cm:broadcast", "/")
		.await?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_redis_sub() {
		redis_sub().await.unwrap();
	}

	#[tokio::test]
	async fn test_redis_pub() {
		redis_pub().await.unwrap();
	}
}
