use anyhow::Context;
use poise::{
	CreateReply,
	serenity_prelude::{
		ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, EmojiId, ReactionType,
		colours::{
			branding::WHITE,
			roles::{GREEN, RED},
		},
	},
};

use crate::{context::Ctx, error::BotResult};

pub const CONFIRM_EMOJI: ReactionType = ReactionType::Custom {
	name: None,
	id: EmojiId::new(1540985370823630858),
	animated: false,
};
pub const DENY_EMOJI: ReactionType = ReactionType::Custom {
	name: None,
	id: EmojiId::new(1540985410597953536),
	animated: false,
};

pub fn confirmation_action_row() -> CreateActionRow {
	let confirm_button = CreateButton::new("confirm")
		.label("Confirm")
		.emoji(CONFIRM_EMOJI)
		.style(ButtonStyle::Success);
	let deny_button = CreateButton::new("deny")
		.label("Deny")
		.emoji(DENY_EMOJI)
		.style(ButtonStyle::Danger);

	CreateActionRow::Buttons(vec![confirm_button, deny_button])
}

pub async fn error_embed(ctx: Ctx<'_>, message: &str) {
	let embed = CreateEmbed::default()
		.title(format!(":x: {message}"))
		.color(RED);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await.ok();
}

pub async fn _success_embed(ctx: Ctx<'_>, message: &str) -> BotResult {
	let embed = CreateEmbed::default()
		.title(format!(":white_check_mark: {message}"))
		.color(GREEN);
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
		.title(":zzz: Shutting down")
		.color(WHITE);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply)
		.await
		.context("Failed to send shutdown embed")?;

	Ok(())
}
