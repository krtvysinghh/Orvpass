use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub theme: String,
    pub compact: bool,
    pub clipboard_timeout: u64,
    pub auto_lock_minutes: u64,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            theme: "premium".to_string(),
            compact: false,
            clipboard_timeout: 30,
            auto_lock_minutes: 15,
        }
    }
}

impl CliConfig {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("orvpass")
            .join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::path();

        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }

        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, data)
    }
}
