use reqwest::Client;

use crate::utils::config::Config;

pub struct Data {
	pub config: Config,
	pub http_client: Client,
}

impl Data {
	pub fn new(config: Config) -> Self {
		Data {
			config,
			http_client: Client::new(),
		}
	}
}
