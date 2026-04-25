use std::sync::Arc;

use crate::app::AppContext;
use crate::storage::ca::{add_issued, build_issuer, load_ca, read_next_serial, IndexRecord};
use crate::storage::cert::save_cert;

use super::FlowError;

pub async fn generate_cert(
    app: &Arc<AppContext>,
    ca_cn: &str,
    email: &str,
) -> Result<(), FlowError> {
    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    let loaded = load_ca(&ca_path).await.ok_or(FlowError::CaNotFound)?;
    let issuer = build_issuer(&loaded).map_err(FlowError::SomethingWentWrong)?;

    let serial = read_next_serial(&ca_path).await;

    let generated = crate::crypto::build_client_cert(&issuer, email, serial)
        .map_err(FlowError::SomethingWentWrong)?;

    save_cert(
        ca_path.clone(),
        email,
        &generated.cert_pem,
        &generated.key_pem,
    )
    .await;

    add_issued(
        &ca_path,
        IndexRecord {
            cn: email.to_string(),
            serial,
            issued_at: time::OffsetDateTime::now_utc().unix_timestamp(),
            revoked_at: None,
        },
    )
    .await;

    Ok(())
}
