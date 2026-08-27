use std::sync::Arc;

use crate::{data::Data, error::BotError};

pub type Ctx<'a> = poise::Context<'a, Arc<Data>, BotError>;
