use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{
		ComponentInteraction, CreateEmbed, CreateInteractionResponse,
		CreateInteractionResponseMessage, Message, colours::branding::WHITE,
	},
};

use crate::{commands::signup::profile::Profile, context::Ctx, error::BotResult, utils::response};

const SKIN_API: &str = "https://nmsr.nickac.dev/fullbodyiso";

pub async fn send_confirmation(ctx: Ctx<'_>, profile: &Profile) -> BotResult<Message> {
	let name = &profile.name;
	let uuid = &profile.uuid;

	let skin_url = format!("{}/{}", SKIN_API, uuid);

	let embed = CreateEmbed::default()
		.title("Is this you?")
		.description(format!("**IGN:** {}\n**Discord:** {}", name, ctx.author().name))
		.thumbnail(skin_url)
		.color(WHITE);
	let row = response::confirmation_action_row();

	let reply = CreateReply::default()
		.embed(embed)
		.ephemeral(true)
		.components(vec![row]);

	let handle = ctx
		.send(reply)
		.await
		.context("Failed to send confirmation embed")?;
	let message = handle
		.into_message()
		.await
		.context("Failed to get message from handle")?;

	Ok(message)
}

pub async fn interaction_confirm(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Request sent")
		.description("Your request has been sent")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx.http(), interaction_response)
		.await
		.context("Failed to respond to confirmation")?;

	Ok(())
}

pub async fn interaction_deny(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Request cancelled")
		.description("Your request has been cancelled")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx.http(), interaction_response)
		.await
		.context("Failed to respond to denial")?;

	Ok(())
}
