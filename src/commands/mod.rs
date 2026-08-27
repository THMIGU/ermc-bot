mod leave;
mod ping;
mod shutdown;
mod signup;

use std::sync::Arc;

use crate::{
	commands::{leave::leave, ping::ping, shutdown::shutdown, signup::signup},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Arc<Data>, BotError>> {
	vec![ping(), shutdown(), signup(), leave()]
}
