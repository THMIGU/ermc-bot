mod commands;
mod context;
mod data;
mod error;
mod services;
mod setup;
mod tasks;
mod utils;

use anyhow::Context;
use poise::serenity_prelude as serenity;

use crate::{
	commands::commands,
	error::{BotResult, on_error},
	utils::config::Config,
};

#[tokio::main]
async fn main() -> BotResult {
	let config: Config = Config::load().context("Failed to load config")?;
	let token = config.discord.token;

	let intents = serenity::GatewayIntents::all();

	let framework_options = poise::FrameworkOptions {
		commands: commands(),
		on_error: |err| Box::pin(on_error(err)),
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(framework_options)
		.setup(|ctx, ready, framework| Box::pin(setup::setup(ctx, ready, framework)))
		.build();

	let mut client = serenity::ClientBuilder::new(&token, intents)
		.framework(framework)
		.await
		.context("Failed to initialize client")?;
	client
		.start()
		.await
		.context("Failed to start client")?;

	println!("Shutting down");

	Ok(())
}
