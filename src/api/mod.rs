pub mod cache;
pub mod client;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub api_key: String,
    pub stats_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: String::new(),
            stats_url: "https://code-opendoor.com/api/me/stats".to_string(),
        }
    }
}

/// OpenDoor /api/me/stats 响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenDoorStats {
    /// 账户余额（人民币）
    pub balance_cny: String,
    /// 今日消费（美元）
    pub used_usd: String,
    /// 总限额（美元）
    pub limit_usd: String,
    /// 消耗百分比 0-100
    pub percentage_used: f64,
    /// 今日调用次数
    pub calls_today: u64,
}

impl OpenDoorStats {
    pub fn is_exhausted(&self) -> bool {
        let limit: f64 = self.limit_usd.parse().unwrap_or(0.0);
        let used: f64 = self.used_usd.parse().unwrap_or(0.0);
        limit > 0.0 && used >= limit
    }

    pub fn balance_cny_f64(&self) -> f64 {
        self.balance_cny.parse().unwrap_or(0.0)
    }

    pub fn used_usd_f64(&self) -> f64 {
        self.used_usd.parse().unwrap_or(0.0)
    }

    pub fn limit_usd_f64(&self) -> f64 {
        self.limit_usd.parse().unwrap_or(0.0)
    }
}

#[derive(Debug, Deserialize)]
struct ClaudeSettings {
    env: Option<ClaudeEnv>,
}

#[derive(Debug, Deserialize)]
struct ClaudeEnv {
    #[serde(rename = "ANTHROPIC_AUTH_TOKEN")]
    auth_token: Option<String>,
    #[serde(rename = "ANTHROPIC_BASE_URL")]
    base_url: Option<String>,
}

fn get_claude_settings_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".claude").join("settings.json"))
}

/// 从 Claude settings.json 读取用户的 API Key
pub fn get_api_key_from_claude_settings() -> Option<String> {
    let path = get_claude_settings_path()?;
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    let settings: ClaudeSettings = serde_json::from_str(&content).ok()?;
    let env = settings.env?;
    if env.base_url.is_some() {
        return env.auth_token;
    }
    None
}

/// 从 Claude settings.json 推断 OpenDoor stats URL
/// base_url 格式通常是 https://xxx.com/v1，去掉 /v1 加 /api/me/stats
pub fn get_stats_url_from_claude_settings() -> Option<String> {
    let path = get_claude_settings_path()?;
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    let settings = serde_json::from_str::<serde_json::Value>(&content).ok()?;
    let base_url = settings.get("env")?.get("ANTHROPIC_BASE_URL")?.as_str()?;

    let base = base_url.trim_end_matches('/');
    let base = base.trim_end_matches("/v1");
    Some(format!("{}/api/me/stats", base))
}
