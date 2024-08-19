use serde::{Deserialize, Serialize};

use crate::temp_dir::TempDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModel {
    pub temp_dir: String,
}

impl SettingsModel {
    pub fn get_temp_dir(&self) -> TempDir {
        let mut result =
            rust_extensions::file_utils::format_path(self.temp_dir.as_str()).to_string();

        if result.ends_with(std::path::MAIN_SEPARATOR) {
            result.pop();
        }

        TempDir::new(result)
    }
}
