use std::sync::Arc;

use anyhow::Context;
use poise::serenity_prelude::{ExecuteWebhook, Webhook};

use crate::{context::TaskCtx, error::BotResult};

pub async fn send_webhook(ctx: Arc<TaskCtx>, builder: ExecuteWebhook) -> BotResult {
	let webhook = Webhook::from_url(
		ctx.http.clone(),
		&ctx.data
			.config
			.discord
			.webhook_url,
	)
	.await
	.context("Failed to create webhook")?;
	webhook
		.execute(ctx.http.clone(), false, builder)
		.await
		.context("Failed to execute webhook")?;

	Ok(())
}
