use crate::{context::Ctx, error::BotResult};

pub async fn send_webhook(ctx: Ctx<'_>, payload: &str) -> BotResult {
	Ok(())
}
