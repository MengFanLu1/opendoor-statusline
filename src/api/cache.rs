use super::OpenDoorStats;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// 缓存有效期：5 分钟
const CACHE_FRESH_SECONDS: u64 = 300;

fn get_cache_file() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let cache_dir = home.join(".claude").join("opendoor-statusline").join("cache");
    fs::create_dir_all(&cache_dir).ok()?;
    Some(cache_dir.join("stats.json"))
}

fn is_cache_fresh(path: &PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                return elapsed.as_secs() < CACHE_FRESH_SECONDS;
            }
        }
    }
    false
}

/// 读取缓存，返回 (数据, 是否需要刷新)
pub fn get_cached_stats() -> (Option<OpenDoorStats>, bool) {
    let path = match get_cache_file() {
        Some(p) => p,
        None => return (None, false),
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return (None, false),
    };

    let data: Option<OpenDoorStats> = serde_json::from_str(&content).ok();
    if data.is_none() {
        return (None, false);
    }

    let fresh = is_cache_fresh(&path);
    (data, !fresh)
}

/// 保存缓存
pub fn save_cached_stats(stats: &OpenDoorStats) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_cache_file() {
        let json = serde_json::to_string(stats)?;
        fs::write(path, json)?;
    }
    Ok(())
}

/// 后台异步刷新缓存
pub fn spawn_background_refresh(api_key: String, stats_url: String) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let config = super::ApiConfig {
            enabled: true,
            api_key,
            stats_url,
        };

        if let Ok(client) = super::client::ApiClient::new(config) {
            if let Ok(stats) = client.get_stats() {
                let _ = save_cached_stats(&stats);
            }
        }
    });
}
