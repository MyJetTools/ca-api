use serde::{Deserialize, Serialize};

use crate::storage::ca::CaDataPath;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRecord {
    pub cn: String,
    pub serial: u64,
    pub issued_at: i64,
    #[serde(default)]
    pub revoked_at: Option<i64>,
}

pub async fn load_index(ca_path: &CaDataPath) -> Vec<IndexRecord> {
    let file = index_file(ca_path);
    match tokio::fs::read(&file).await {
        Ok(bytes) if !bytes.is_empty() => serde_json::from_slice(&bytes).unwrap_or_default(),
        _ => Vec::new(),
    }
}

pub async fn save_index(ca_path: &CaDataPath, index: &[IndexRecord]) {
    let file = index_file(ca_path);
    let bytes = serde_json::to_vec_pretty(index).unwrap();
    tokio::fs::write(&file, bytes).await.unwrap();
}

pub async fn add_issued(ca_path: &CaDataPath, record: IndexRecord) {
    let mut index = load_index(ca_path).await;
    if let Some(existing) = index.iter_mut().find(|r| r.cn == record.cn) {
        *existing = record;
    } else {
        index.push(record);
    }
    save_index(ca_path, &index).await;
}

pub async fn mark_revoked(
    ca_path: &CaDataPath,
    cn: &str,
    when: i64,
) -> Option<IndexRecord> {
    let mut index = load_index(ca_path).await;
    let mut updated = None;
    for r in index.iter_mut() {
        if r.cn == cn && r.revoked_at.is_none() {
            r.revoked_at = Some(when);
            updated = Some(r.clone());
            break;
        }
    }
    if updated.is_some() {
        save_index(ca_path, &index).await;
    }
    updated
}

pub fn index_file(ca_path: &CaDataPath) -> String {
    let mut path = ca_path.as_str().to_string();
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str("index.json");
    path
}
