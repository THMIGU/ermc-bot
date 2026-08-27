use std::sync::Arc;

use poise::serenity_prelude::Http;

use crate::{data::Data, error::BotError};

pub type Ctx<'a> = poise::Context<'a, Arc<Data>, BotError>;

pub struct TaskCtx {
	pub data: Arc<Data>,
	pub http: Arc<Http>,
}
