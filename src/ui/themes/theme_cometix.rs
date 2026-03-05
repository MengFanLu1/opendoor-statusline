use crate::config::{
    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn model_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Model,
        enabled: true,
        icon: IconConfig {
            plain: "🤖".to_string(),
            nerd_font: "\u{e26d}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }),
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn directory_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Directory,
        enabled: true,
        icon: IconConfig {
            plain: "📁".to_string(),
            nerd_font: "\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 11 }),
            text: Some(AnsiColor::Color16 { c16: 10 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn git_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Git,
        enabled: true,
        icon: IconConfig {
            plain: "🌿".to_string(),
            nerd_font: "\u{f02a2}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 12 }),
            text: Some(AnsiColor::Color16 { c16: 12 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: {
            let mut opts = HashMap::new();
            opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
            opts
        },
    }
}

pub fn context_window_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::ContextWindow,
        enabled: true,
        icon: IconConfig {
            plain: "⚡️".to_string(),
            nerd_font: "\u{f49b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 13 }),
            text: Some(AnsiColor::Color16 { c16: 13 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cost,
        enabled: false,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{eec1}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 3 }),
            text: Some(AnsiColor::Color16 { c16: 3 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn session_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Session,
        enabled: false,
        icon: IconConfig {
            plain: "⏱️".to_string(),
            nerd_font: "\u{f19bb}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 2 }),
            text: Some(AnsiColor::Color16 { c16: 2 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn output_style_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OutputStyle,
        enabled: false,
        icon: IconConfig {
            plain: "🎯".to_string(),
            nerd_font: "\u{f12f5}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 6 }),
            text: Some(AnsiColor::Color16 { c16: 6 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage,
        enabled: false,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }),
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "api_base_url".to_string(),
                serde_json::Value::String("https://api.anthropic.com".to_string()),
            );
            opts.insert(
                "cache_duration".to_string(),
                serde_json::Value::Number(180.into()),
            );
            opts.insert("timeout".to_string(), serde_json::Value::Number(2.into()));
            opts
        },
    }
}

pub fn opendoor_usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OpenDoorUsage,
        enabled: true,
        icon: IconConfig {
            plain: "opendoor".to_string(),
            nerd_font: "\u{f0690}".to_string(), // nf-md-gauge
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 214 }), // Orange
            text: Some(AnsiColor::Color256 { c256: 255 }), // White
            background: None,
        },
        styles: TextStyleConfig { text_bold: false },
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "api_key".to_string(),
                serde_json::Value::String("".to_string()),
            );
            opts
        },
    }
}

pub fn opendoor_subscription_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OpenDoorSubscription,
        enabled: true,
        icon: IconConfig {
            plain: "订阅".to_string(),
            nerd_font: "\u{f0e21}".to_string(), // nf-md-crown
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 39 }),  // Blue
            text: Some(AnsiColor::Color256 { c256: 255 }), // White
            background: None,
        },
        styles: TextStyleConfig { text_bold: false },
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "api_key".to_string(),
                serde_json::Value::String("".to_string()),
            );
            opts
        },
    }
}

pub fn opendoor_status_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OpenDoorStatus,
        enabled: true, // 默认启用
        icon: IconConfig {
            plain: "".to_string(), // 无图标,直接显示文字
            nerd_font: "".to_string(),
        },
        colors: ColorConfig {
            icon: None,
            text: None, // 颜色由segment自己控制(彩色文字效果)
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
