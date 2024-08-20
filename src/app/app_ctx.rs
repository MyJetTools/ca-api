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

    pub fn get_easy_rsa_command(&self) -> String {
        "/usr/share/easy-rsa/easyrsa".to_string()
    }

    pub fn get_vars_path(&self) -> &str {
        "/usr/share/easy-rsa/vars"
    }

    pub fn get_pki_path(&self) -> &str {
        "/usr/share/easy-rsa/pki"
    }

    pub fn get_private_dir(&self) -> &str {
        "/usr/share/easy-rsa/pki/private"
    }

    pub fn get_reqs_dir(&self) -> &str {
        "/usr/share/easy-rsa/pki/reqs"
    }

    pub fn get_client_cert_private_key_file(&self, email: &str) -> String {
        format!("/usr/share/easy-rsa/pki/private/{}.key", email)
    }

    pub fn get_client_cert_file(&self, email: &str) -> String {
        format!("/usr/share/easy-rsa/pki/issued/{}.crt", email)
    }

    pub fn get_crl_file(&self) -> String {
        format!("/usr/share/easy-rsa/pki/crl.pem")
    }

    pub fn get_client_cert_pfx_file(&self, email: &str) -> String {
        let mut temp_dir = self.settings.get_temp_dir().into_string();
        temp_dir.push_str(format!("/{}.pfx", email).as_str());

        temp_dir
    }
}
