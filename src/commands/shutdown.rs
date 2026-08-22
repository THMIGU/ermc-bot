use poise::serenity_prelude as serenity;

use crate::{
	context::Ctx,
	error::BotResult,
	utils::{checks::is_owner, response},
};

/// Shuts down ERMC Bot (owner only).
#[poise::command(slash_command, check = "is_owner")]
pub async fn shutdown(ctx: Ctx<'_>) -> BotResult {
	response::shutdown_embed(ctx).await?;

	ctx.serenity_context()
		.set_presence(None, serenity::OnlineStatus::Offline);

	let shard_manager = ctx
		.framework()
		.shard_manager
		.clone();
	shard_manager
		.shutdown_all()
		.await;

	Ok(())
}
