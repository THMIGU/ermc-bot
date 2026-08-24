use anyhow::Context;
use rusqlite::{Connection, OptionalExtension, params};

use crate::error::BotResult;

const DATABASE_PATH: &str = "data.db";

fn get_connection() -> BotResult<Connection> {
	Connection::open(DATABASE_PATH).context(format!("Failed to open {}", DATABASE_PATH))
}

pub fn init_db() -> BotResult {
	let db = get_connection()?;

	db.execute(
		"CREATE TABLE IF NOT EXISTS players (
			discord_id INTEGER PRIMARY KEY,
			ign TEXT NOT NULL,
			uuid TEXT NOT NULL UNIQUE
		)",
		[],
	)
	.context("Failed to create player table")?;

	Ok(())
}

pub fn add_player(discord_id: u64, ign: &str, uuid: &str) -> BotResult {
	let db = get_connection()?;

	db.execute(
		"INSERT INTO players (discord_id, ign, uuid)
		VALUES (?1, ?2, ?3)",
		params![discord_id as i64, ign, uuid],
	)
	.context("Failed to add player to table")?;

	Ok(())
}

pub fn get_player(discord_id: u64) -> BotResult<Option<(String, String)>> {
	let db = get_connection()?;

	let mut stmt = db.prepare(
		"SELECT ign, uuid
		FROM players
		WHERE discord_id = ?1",
	)?;

	let player = stmt
		.query_row(params![discord_id as i64], |row| {
			let ign: String = row.get(0)?;
			let uuid: String = row.get(1)?;

			Ok((ign, uuid))
		})
		.optional()
		.context("Failed to get ID in table")?;

	Ok(player)
}
