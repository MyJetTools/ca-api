use crate::settings::ConfigPath;

pub async fn list_cas(config_path: &ConfigPath) -> Vec<String> {
    let mut result = Vec::new();
    let mut entries = match tokio::fs::read_dir(config_path.as_str()).await {
        Ok(e) => e,
        Err(_) => return result,
    };

    while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
        let Ok(file_type) = entry.file_type().await else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }

        let name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let mut ca_cert = entry.path();
        ca_cert.push("ca_cert.pem");
        if tokio::fs::metadata(&ca_cert).await.is_ok() {
            result.push(name);
        }
    }

    result
}
