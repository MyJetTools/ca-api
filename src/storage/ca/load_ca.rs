use rcgen::{Issuer, KeyPair};

use crate::storage::ca::CaDataPath;

pub struct LoadedCa {
    pub cert_pem: String,
    pub key_pem: String,
}

pub async fn load_ca(ca_path: &CaDataPath) -> Option<LoadedCa> {
    let cert_pem = tokio::fs::read_to_string(ca_path.to_ca_cert_file_name())
        .await
        .ok()?;
    let key_pem = tokio::fs::read_to_string(ca_path.to_ca_private_key_file_name())
        .await
        .ok()?;
    Some(LoadedCa { cert_pem, key_pem })
}

pub fn build_issuer(loaded: &LoadedCa) -> Result<Issuer<'static, KeyPair>, String> {
    let key_pair = KeyPair::from_pem(&loaded.key_pem).map_err(|e| e.to_string())?;
    Issuer::from_ca_cert_pem(&loaded.cert_pem, key_pair).map_err(|e| e.to_string())
}
