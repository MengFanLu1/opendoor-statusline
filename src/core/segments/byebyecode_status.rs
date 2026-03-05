use crate::config::{Config, InputData, SegmentId};
use crate::core::segments::SegmentData;
use crate::api::cache;
use std::collections::HashMap;

pub fn collect(config: &Config, _input: &InputData) -> Option<SegmentData> {
    let segment = config
        .segments
        .iter()
        .find(|s| matches!(s.id, SegmentId::OpenDoorStatus))?;

    if !segment.enabled {
        return None;
    }

    let (stats, _) = cache::get_cached_stats();

    let primary = match stats {
        Some(s) if s.is_exhausted() => "⚠ 余额不足".to_string(),
        Some(s) => format!("¥{:.2}", s.balance_cny_f64()),
        None => "OpenDoor".to_string(),
    };

    Some(SegmentData {
        primary,
        secondary: String::new(),
        metadata: HashMap::new(),
    })
}
