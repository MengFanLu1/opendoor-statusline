use crate::config::{Config, InputData, SegmentId};
use crate::core::segments::SegmentData;
use crate::api::cache;
use std::collections::HashMap;

pub fn collect(config: &Config, _input: &InputData) -> Option<SegmentData> {
    let segment = config
        .segments
        .iter()
        .find(|s| matches!(s.id, SegmentId::OpenDoorSubscription))?;

    if !segment.enabled {
        return None;
    }

    let (stats, _) = cache::get_cached_stats();

    let primary = match stats {
        Some(s) => format!(
            "${:.2}/{:.0} ({:.1}%)",
            s.used_usd_f64(),
            s.limit_usd_f64(),
            s.percentage_used
        ),
        None => "—".to_string(),
    };

    Some(SegmentData {
        primary,
        secondary: String::new(),
        metadata: HashMap::new(),
    })
}
