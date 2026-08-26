use anyhow::Context;
use poise::{
	Framework,
	serenity_prelude::{Context as SerenityContext, Ready},
};

use crate::{
	data::Data,
	error::{BotError, BotResult},
	utils::{config::Config, database},
};

pub async fn setup(
	ctx: &SerenityContext,
	ready: &Ready,
	framework: &Framework<Data, BotError>,
) -> BotResult<Data> {
	let config: Config = Config::load().context("Failed to load config")?;

	poise::builtins::register_globally(ctx, &framework.options().commands)
		.await
		.context("Failed to register commands")?;
	println!("Logged in as {}", ready.user.tag());

	database::init_db().context("Failed to initialize database")?;

	let data = Data::new(config)
		.await
		.context("Failed to initialize data")?;
	Ok(data)
}
