use std::sync::Arc;

use crate::app::AppContext;
use crate::crypto::CrlEntry;
use crate::storage::ca::{build_issuer, load_ca, load_index, read_crl_number};

use super::FlowError;

pub async fn get_crl(app: &Arc<AppContext>, ca_cn: &str) -> Result<String, FlowError> {
    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    if let Ok(content) = tokio::fs::read_to_string(ca_path.to_crl_file_name()).await {
        return Ok(content);
    }

    let loaded = load_ca(&ca_path).await.ok_or(FlowError::CaNotFound)?;
    let issuer = build_issuer(&loaded).map_err(FlowError::SomethingWentWrong)?;

    let index = load_index(&ca_path).await;
    let revoked: Vec<CrlEntry> = index
        .iter()
        .filter_map(|r| {
            r.revoked_at.map(|ts| CrlEntry {
                serial: r.serial,
                revoked_at: time::OffsetDateTime::from_unix_timestamp(ts)
                    .unwrap_or(time::OffsetDateTime::UNIX_EPOCH),
            })
        })
        .collect();

    let crl_number = read_crl_number(&ca_path).await;
    let crl_pem = crate::crypto::build_crl(&issuer, &revoked, crl_number)
        .map_err(FlowError::SomethingWentWrong)?;

    tokio::fs::write(ca_path.to_crl_file_name(), &crl_pem)
        .await
        .map_err(|e| FlowError::SomethingWentWrong(e.to_string()))?;

    Ok(crl_pem)
}
