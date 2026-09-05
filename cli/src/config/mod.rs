pub mod interactive;

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub color: bool,
    pub quiet: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            color: true,
            quiet: false,
        }
    }
}

pub fn export_config_json() -> String {
    "{\"clipboard_timeout\": 15, \"theme\": \"TokyoNight\"}".to_string()
}
