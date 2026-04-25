use std::sync::Arc;

use crate::app::AppContext;
use crate::storage::ca::load_ca;
use crate::storage::cert::load_cert;

use super::FlowError;

pub async fn get_p12(
    app: &Arc<AppContext>,
    ca_cn: &str,
    email: &str,
    password: &str,
) -> Result<Vec<u8>, FlowError> {
    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    let ca = load_ca(&ca_path).await.ok_or(FlowError::CaNotFound)?;

    let client = load_cert(ca_path, email)
        .await
        .ok_or(FlowError::CertNotFound)?;

    crate::crypto::build_p12(
        client.cert_pem.as_bytes(),
        client.key_pem.as_bytes(),
        ca.cert_pem.as_bytes(),
        email,
        password,
    )
    .map_err(FlowError::SomethingWentWrong)
}
