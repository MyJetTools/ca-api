use crate::storage::ca::CaDataPath;
use crate::storage::cert::ClientCertPath;

pub async fn save_cert(ca_path: CaDataPath, email: &str, cert_pem: &str, key_pem: &str) {
    let cert_path = ClientCertPath::from_ca_path(ca_path, email);

    tokio::fs::create_dir_all(cert_path.as_str()).await.unwrap();

    tokio::fs::write(cert_path.to_cert_file_name(), cert_pem)
        .await
        .unwrap();

    tokio::fs::write(cert_path.to_private_key_file_name(), key_pem)
        .await
        .unwrap();
}
