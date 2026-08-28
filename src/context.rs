use std::sync::Arc;

use poise::serenity_prelude::Http;

use crate::{data::Data, error::BotError};

pub type Ctx<'a> = poise::Context<'a, Arc<Data>, BotError>;

pub struct TaskCtx {
	pub data: Arc<Data>,
	pub http: Arc<Http>,
}

impl From<Ctx<'_>> for TaskCtx {
	fn from(value: Ctx<'_>) -> Self {
		let data = value.data();

		Self {
			data: data.clone(),
			http: value
				.serenity_context()
				.http
				.clone(),
		}
	}
}
