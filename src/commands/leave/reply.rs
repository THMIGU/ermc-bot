use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{
		ComponentInteraction, CreateEmbed, CreateInteractionResponse,
		CreateInteractionResponseMessage, Message,
		colours::branding::{RED, WHITE},
	},
};

use crate::{context::Ctx, error::BotResult, utils::response};

pub async fn send_confirmation(ctx: Ctx<'_>) -> BotResult<Message> {
	let embed = CreateEmbed::default()
		.title("Leave?")
		.description("Please confirm that you want to leave")
		.color(RED);
	let row = response::confirmation_action_row();

	let reply = CreateReply::default()
		.embed(embed)
		.components(vec![row]);

	let handle = ctx
		.send(reply)
		.await
		.context("Failed to reply to leave")?;
	let message = handle
		.into_message()
		.await
		.context("Failed to get message from handle")?;

	Ok(message)
}

pub async fn interaction_confirm(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Removal confirmed")
		.description("You have been removed from the whitelist")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to confirmation")?;

	Ok(())
}

pub async fn interaction_deny(ctx: Ctx<'_>, interaction: ComponentInteraction) -> BotResult {
	let embed = CreateEmbed::default()
		.title("Removal cancelled")
		.description("You were not removed from the whitelist")
		.color(WHITE);
	let message = CreateInteractionResponseMessage::new()
		.embed(embed)
		.components(vec![]);

	let interaction_response = CreateInteractionResponse::UpdateMessage(message);

	interaction
		.create_response(ctx, interaction_response)
		.await
		.context("Failed to respond to denial")?;

	Ok(())
}
