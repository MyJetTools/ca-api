use std::sync::Arc;

use my_tls::crl::CrlRecord;

use crate::app::AppContext;

pub async fn get_list_of_revoked_certificates(
    app: &Arc<AppContext>,
    ca_cn: &str,
) -> Vec<CrlRecord> {
    let path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    match tokio::fs::read(path.to_crl_file_name()).await {
        Ok(crl_content) => my_tls::crl::read(&crl_content).unwrap(),
        Err(_) => Vec::new(),
    }
}
