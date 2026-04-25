use std::sync::Arc;

use crate::app::AppContext;
use crate::storage::ca::load_index;

pub struct IssuedCertificateInfo {
    pub cn: String,
    pub revoked: bool,
}

pub async fn get_list_of_issued_certificates(
    app: &Arc<AppContext>,
    ca_cn: &str,
) -> Vec<IssuedCertificateInfo> {
    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    load_index(&ca_path)
        .await
        .into_iter()
        .map(|r| IssuedCertificateInfo {
            cn: r.cn,
            revoked: r.revoked_at.is_some(),
        })
        .collect()
}
