use crate::api::{cache, client::ApiClient, ApiConfig};
use crate::config::{Config, InputData, SegmentId};
use crate::core::segments::SegmentData;
use std::collections::HashMap;

const RESET: &str = "\x1b[0m";

fn get_status_color(percentage: f64) -> &'static str {
    if percentage <= 50.0 {
        "\x1b[38;5;114m" // 柔和绿
    } else if percentage <= 80.0 {
        "\x1b[38;5;179m" // 柔和黄
    } else {
        "\x1b[38;5;167m" // 柔和红
    }
}

pub fn collect(config: &Config, _input: &InputData) -> Option<SegmentData> {
    let segment = config
        .segments
        .iter()
        .find(|s| matches!(s.id, SegmentId::OpenDoorUsage))?;

    if !segment.enabled {
        return None;
    }

    // 优先从 segment options 读取，否则从 Claude settings 推断
    let stats_url = segment
        .options
        .get("stats_url")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(crate::api::get_stats_url_from_claude_settings)
        .unwrap_or_else(|| "https://code-opendoor.com/api/me/stats".to_string());

    let api_key = segment
        .options
        .get("api_key")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(crate::api::get_api_key_from_claude_settings);

    let api_key = match api_key {
        Some(k) if !k.is_empty() => k,
        _ => {
            return Some(SegmentData {
                primary: "未配置 API Key".to_string(),
                secondary: String::new(),
                metadata: HashMap::new(),
            });
        }
    };

    // 先尝试 API，失败降级到缓存
    let stats = fetch_stats_with_cache(&api_key, &stats_url);

    let stats = match stats {
        Some(s) => s,
        None => {
            return Some(SegmentData {
                primary: "⏳ 获取中...".to_string(),
                secondary: String::new(),
                metadata: HashMap::new(),
            });
        }
    };

    let used = stats.used_usd_f64();
    let limit = stats.limit_usd_f64();
    let pct = stats.percentage_used;
    let safe_pct = if pct.is_finite() {
        pct.clamp(0.0, 100.0)
    } else {
        0.0
    };

    let mut metadata = HashMap::new();
    metadata.insert("used_usd".to_string(), stats.used_usd.clone());
    metadata.insert("limit_usd".to_string(), stats.limit_usd.clone());

    // 额度耗尽
    if stats.is_exhausted() {
        return Some(SegmentData {
            primary: format!("${:.2}/{:.0} 已耗尽", used, limit),
            secondary: "请前往 OpenDoor 充值".to_string(),
            metadata,
        });
    }

    // 生成进度条
    let bar_len = 8usize;
    let filled = ((safe_pct / 100.0) * bar_len as f64).round() as usize;
    let empty = bar_len.saturating_sub(filled);
    let color = get_status_color(safe_pct);
    let bar = format!("{}{}{}{}", color, "▓".repeat(filled), "░".repeat(empty), RESET);

    Some(SegmentData {
        primary: format!("${:.2}/{:.0} {}", used, limit, bar),
        secondary: String::new(),
        metadata,
    })
}

fn fetch_stats_with_cache(
    api_key: &str,
    stats_url: &str,
) -> Option<crate::api::OpenDoorStats> {
    let config = ApiConfig {
        enabled: true,
        api_key: api_key.to_string(),
        stats_url: stats_url.to_string(),
    };

    if let Ok(client) = ApiClient::new(config) {
        if let Ok(stats) = client.get_stats() {
            let _ = cache::save_cached_stats(&stats);
            return Some(stats);
        }
    }

    // API 失败，降级到缓存
    let (cached, _) = cache::get_cached_stats();
    cached
}
