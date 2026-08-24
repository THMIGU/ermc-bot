mod reply;

use anyhow::Context;

use crate::{
	context::Ctx,
	error::BotResult,
	utils::{database, response},
};

/// Remove yourself from the ERMC whitelist.
#[poise::command(slash_command)]
pub async fn leave(ctx: Ctx<'_>) -> BotResult {
	ctx.defer_ephemeral()
		.await
		.context("Failed to defer response")?;

	if database::get_player(ctx.author().id.get())
		.context("Failed to get player from database")?
		.is_none()
	{
		response::error_embed(ctx, "You have not signed up").await;
		return Ok(());
	};

	let message = reply::send_confirmation(ctx).await?;

	let Some(interaction) = message
		.await_component_interaction(ctx)
		.await
	else {
		return Ok(());
	};

	match interaction
		.data
		.custom_id
		.as_str()
	{
		"confirm" => reply::user_confirmed(ctx, interaction).await?,
		"deny" => {
			reply::user_denied(ctx, interaction).await?;
			return Ok(());
		}
		_ => return Ok(()),
	};

	database::remove_player(ctx.author().id.get())?;

	Ok(())
}
