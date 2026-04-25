use crate::storage::ca::CaDataPath;

pub async fn save_ca(ca_path: &CaDataPath, ca_cert_pem: &str, ca_key_pem: &str) {
    tokio::fs::create_dir_all(ca_path.as_str()).await.unwrap();

    let mut certs_dir = ca_path.as_str().to_string();
    if !certs_dir.ends_with('/') {
        certs_dir.push('/');
    }
    certs_dir.push_str("certs");
    tokio::fs::create_dir_all(&certs_dir).await.unwrap();

    tokio::fs::write(ca_path.to_ca_cert_file_name(), ca_cert_pem)
        .await
        .unwrap();

    tokio::fs::write(ca_path.to_ca_private_key_file_name(), ca_key_pem)
        .await
        .unwrap();

    tokio::fs::write(ca_path.to_serial_file_name(), "1")
        .await
        .unwrap();

    crate::storage::ca::save_index(ca_path, &[]).await;
}
