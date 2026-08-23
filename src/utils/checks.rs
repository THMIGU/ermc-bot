use anyhow::Context;

use crate::{context::Ctx, error::BotResult};

pub async fn is_owner(ctx: Ctx<'_>) -> BotResult<bool> {
	Ok(ctx.author().id
		== ctx
			.data()
			.config
			.discord
			.owner_id)
}

pub async fn is_essress(ctx: Ctx<'_>) -> BotResult<bool> {
	Ok(ctx
		.guild()
		.context("Message guild not found")?
		.id == ctx
		.data()
		.config
		.discord
		.essress_id)
}
