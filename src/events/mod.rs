mod on_message;

use std::sync::Arc;

use poise::serenity_prelude::{Context, FullEvent};

use crate::{data::Data, error::BotResult, events::on_message::on_message};

pub async fn event_handler(ctx: &Context, event: &FullEvent, data: &Arc<Data>) -> BotResult {
	match event {
		FullEvent::Message {
			new_message,
		} => on_message(ctx, data, new_message).await?,
		_ => (),
	}

	Ok(())
}
