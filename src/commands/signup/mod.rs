mod profile;
mod reply;

use anyhow::Context;

use crate::{
	context::Ctx,
	error::BotResult,
	services::redis,
	utils::{checks::is_essress, database, response},
};

/// Sign up to be on the whitelist for ERMC.
#[poise::command(slash_command, check = "is_essress")]
pub async fn signup(ctx: Ctx<'_>, #[description = "Your Minecraft IGN."] ign: String) -> BotResult {
	ctx.defer_ephemeral()
		.await
		.context("Failed to defer response")?;

	let discord_id = ctx.author().id.get();
	if database::check_requests(discord_id)? {
		response::error_embed(ctx, "You have already sent a request").await;
		return Ok(());
	}

	let profile = profile::get_profile(&ctx.data().http_client, &ign)
		.await
		.context("Failed to get profile")?;

	let Some(profile) = profile else {
		response::error_embed(ctx, &format!("Could not find account named \"{}\"", ign)).await;
		return Ok(());
	};

	match database::validate_player(discord_id, &profile.name, &profile.uuid) {
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
		"confirm" => reply::user_confirmed(ctx, interaction).await?,
		"deny" => {
			reply::user_denied(ctx, interaction).await?;
			return Ok(());
		}
		_ => return Ok(()),
	};

	let message = reply::send_request(ctx, &profile).await?;
	database::add_request(discord_id)?;

	let Some(interaction) = message
		.await_component_interaction(ctx)
		.await
	else {
		return Ok(());
	};

	database::remove_request(discord_id)?;

	match interaction
		.data
		.custom_id
		.as_str()
	{
		"confirm" => reply::owner_confirmed(ctx, interaction).await?,
		"deny" => {
			reply::owner_denied(ctx, interaction).await?;
			reply::send_denied(ctx, discord_id).await?;
			return Ok(());
		}
		_ => return Ok(()),
	}

	reply::send_confirmed(ctx, discord_id).await?;
	database::add_player(discord_id, &profile.name, &profile.uuid)?;
	redis::redis_pub(ctx.data().clone(), "ermc:whitelist:update", &profile.uuid).await?;

	Ok(())
}
