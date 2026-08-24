mod profile;
mod reply;

use anyhow::Context;
use poise::serenity_prelude::UserId;

use crate::{
	context::Ctx,
	error::BotResult,
	utils::{checks::is_essress, database, response},
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

	match database::validate_player(ctx.author().id.get(), &profile.name, &profile.uuid) {
		Ok(_) => (),
		Err(_) => {
			response::error_embed(
				ctx,
				"You have already signed up or someone has already signed up with that account",
			)
			.await;

			return Ok(());
		}
	}

	let message = reply::send_confirmation(ctx, &profile).await?;

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
		"confirm" => reply::interaction_confirm(ctx, interaction).await?,
		"deny" => {
			reply::interaction_deny(ctx, interaction).await?;
			return Ok(());
		}
		_ => return Ok(()),
	};

	let config = &ctx.data().config;

	let owner_id = UserId::new(config.discord.owner_id);
	let owner = owner_id
		.to_user(ctx)
		.await
		.context("Failed to get user from owner ID")?;

	

	Ok(())
}
