use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{CreateEmbed, colours::branding::WHITE},
};

use crate::{context::Ctx, error::BotResult};

pub async fn error_embed(ctx: Ctx<'_>, message: &str) {
	let embed = CreateEmbed::default()
		.title(format!(":x: {message}"))
		.color(WHITE);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await.ok();
}

pub async fn success_embed(ctx: Ctx<'_>, message: &str) -> BotResult {
	let embed = CreateEmbed::default()
		.title(format!(":white_check_mark: {message}"))
		.color(WHITE);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send success embed")?;

	Ok(())
}

pub async fn ping_embed(ctx: Ctx<'_>) -> BotResult {
	let embed = CreateEmbed::default()
		.title(":ping_pong: Pong!")
		.color(WHITE);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send ping embed")?;

	Ok(())
}

pub async fn shutdown_embed(ctx: Ctx<'_>) -> BotResult {
	let embed = CreateEmbed::default()
		.title(":zzz: Shutting down!")
		.color(WHITE);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send shutdown embed")?;

	Ok(())
}
