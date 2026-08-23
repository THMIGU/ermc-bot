mod uuid;

use anyhow::Context;

use crate::{
	context::Ctx,
	error::BotResult,
	utils::{checks::is_essress, response},
};

/// Sign up to be on the whitelist for ERMC.
#[poise::command(slash_command, check = "is_essress")]
pub async fn signup(ctx: Ctx<'_>, #[description = "Your Minecraft IGN."] ign: String) -> BotResult {
	ctx.defer()
		.await
		.context("Failed to defer response")?;

	response::ping_embed(ctx).await?;

	Ok(())
}
