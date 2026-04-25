use crate::storage::ca::CaDataPath;
use crate::storage::cert::ClientCertPath;

pub struct LoadedClientCert {
    pub cert_pem: String,
    pub key_pem: String,
}

pub async fn load_cert(ca_path: CaDataPath, email: &str) -> Option<LoadedClientCert> {
    let cert_path = ClientCertPath::from_ca_path(ca_path, email);

    let cert_pem = tokio::fs::read_to_string(cert_path.to_cert_file_name())
        .await
        .ok()?;
    let key_pem = tokio::fs::read_to_string(cert_path.to_private_key_file_name())
        .await
        .ok()?;

    Some(LoadedClientCert { cert_pem, key_pem })
}
