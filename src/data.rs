use anyhow::Context;
use redis::{Client as RedisClient, aio::ConnectionManager};
use reqwest::Client as HttpClient;

use crate::{error::BotResult, utils::config::Config};

pub struct Data {
	pub config: Config,
	pub http_client: HttpClient,
	pub redis_client: RedisClient,
	pub redis_manager: ConnectionManager,
}

impl Data {
	pub async fn new(config: Config) -> BotResult<Self> {
		let http_client = HttpClient::new();

		let redis_ip = &config.redis.ip;
		let redis_port = config.redis.port;

		let redis_client = RedisClient::open(format!("redis://{redis_ip}:{redis_port}/"))
			.context("Failed to initialize redis client")?;
		let redis_manager = redis_client
			.get_connection_manager()
			.await
			.context("Failed to get connection manager")?;

		Ok(Data {
			config,
			http_client,
			redis_client,
			redis_manager,
		})
	}
}
