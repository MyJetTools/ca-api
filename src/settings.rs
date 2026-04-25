use serde::{Deserialize, Serialize};

use crate::storage::ca::CaDataPath;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModel {
    pub config_path: String,
}

impl SettingsModel {
    pub fn get_config_path(&self) -> ConfigPath {
        let mut path =
            rust_extensions::file_utils::format_path(self.config_path.as_str()).to_string();

        if path.ends_with(std::path::MAIN_SEPARATOR) {
            path.pop();
        }

        ConfigPath { path }
    }
}

pub struct ConfigPath {
    path: String,
}

impl ConfigPath {
    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn into_ca_data_path(self, ca_cn: &str) -> CaDataPath {
        CaDataPath::new(self.path, ca_cn)
    }
}
