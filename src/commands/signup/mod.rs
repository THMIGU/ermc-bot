mod profile;
mod reply;

use anyhow::Context;

use crate::{
	context::Ctx,
	error::BotResult,
	utils::{checks::is_essress, response},
};

/// Sign up to be on the whitelist for ERMC.
#[poise::command(slash_command, check = "is_essress")]
pub async fn signup(ctx: Ctx<'_>, #[description = "Your Minecraft IGN."] ign: String) -> BotResult {
	ctx.defer_ephemeral()
		.await
		.context("Failed to defer response")?;

	let profile = profile::get_profile(&ctx.data().http_client, &ign)
		.await
		.context("Failed to get profile")?;

	let Some(profile) = profile else {
		response::error_embed(ctx, &format!("Could not find account named \"{}\"", ign)).await;
		return Ok(());
	};

	let message = reply::send_confirmation(ctx, &profile).await?;
	if let Some(interaction) = message
		.await_component_interaction(ctx)
		.await
	{
		match interaction
			.data
			.custom_id
			.as_str()
		{
			"confirm" => reply::interaction_confirm(ctx, interaction).await?,
			"deny" => reply::interaction_deny(ctx, interaction).await?,
			_ => (),
		};
	}

	Ok(())
}
