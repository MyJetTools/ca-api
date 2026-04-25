use std::sync::Arc;

use my_settings_reader::SettingsReader;
use rust_extensions::AppStates;

use crate::settings::SettingsModel;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: Arc<SettingsModel>,
}

impl AppContext {
    pub async fn new() -> Self {
        let settings_reader: SettingsReader<SettingsModel> = SettingsReader::new("~/.ca-api");
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            settings: settings_reader.get_settings().await,
        }
    }
}
