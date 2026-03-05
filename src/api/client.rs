use super::{ApiConfig, OpenDoorStats};
use reqwest::blocking::Client;
use std::time::Duration;

pub struct ApiClient {
    config: ApiConfig,
    client: Client,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(8))
            .user_agent("opendoor-statusline/1.0.0")
            .build()?;

        Ok(Self { config, client })
    }

    /// 调用 OpenDoor /api/me/stats 接口获取余额和用量
    pub fn get_stats(&self) -> Result<OpenDoorStats, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&self.config.stats_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()?;

        if !response.status().is_success() {
            return Err(format!("OpenDoor API error: {}", response.status()).into());
        }

        let stats: OpenDoorStats = response.json()?;
        Ok(stats)
    }
}
