use crate::config::{Config, InputData, SegmentId};
use crate::core::segments::SegmentData;
use std::collections::HashMap;

/// 今日消费段：显示当日消费金额和调用次数
pub fn collect(config: &Config, _input: &InputData) -> Option<SegmentData> {
    let segment = config
        .segments
        .iter()
        .find(|s| matches!(s.id, SegmentId::OpenDoorDailyCost))?;

    if !segment.enabled {
        return None;
    }

    // 复用 opendoor_usage 的 API Key 和 URL 读取逻辑
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

    // 复用 opendoor_usage 的缓存请求函数
    let stats = super::opendoor_usage::fetch_stats_with_cache(&api_key, &stats_url);

    let stats = match stats {
        Some(s) => s,
        None => {
            return Some(SegmentData {
                primary: "获取中...".to_string(),
                secondary: String::new(),
                metadata: HashMap::new(),
            });
        }
    };

    let cost_today = stats.cost_today_usd_f64();
    let calls = stats.calls_today;

    Some(SegmentData {
        primary: format!("${:.2} ({}次)", cost_today, calls),
        secondary: String::new(),
        metadata: HashMap::new(),
    })
}
