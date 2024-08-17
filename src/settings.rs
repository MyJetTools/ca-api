use serde::{Deserialize, Serialize};

use crate::config_path::ConfigPath;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModel {
    pub db: String,
}

impl SettingsModel {
    pub fn get_config_path(&self) -> ConfigPath {
        let mut result = rust_extensions::file_utils::format_path(self.db.as_str()).to_string();

        if !result.ends_with(std::path::MAIN_SEPARATOR) {
            result.push(std::path::MAIN_SEPARATOR)
        }

        ConfigPath::new(result)
    }
}
