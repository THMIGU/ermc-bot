mod ping;
mod shutdown;
mod signup;

use crate::{
	commands::{ping::ping, shutdown::shutdown},
	data::Data,
	error::BotError,
};

pub fn commands() -> Vec<poise::Command<Data, BotError>> {
	vec![ping(), shutdown()]
}
