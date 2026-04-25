use crate::app::AppContext;
use crate::flows::FlowError;

pub async fn import(
    app: &AppContext,
    ca_cn: &str,
    private_key: Vec<u8>,
    cert: Vec<u8>,
) -> Result<(), FlowError> {
    crate::scripts::check_if_we_already_generated_ca(app, ca_cn).await?;

    let cert_pem = String::from_utf8(cert)
        .map_err(|e| FlowError::ValidationError(format!("Invalid certificate: {}", e)))?;
    let key_pem = String::from_utf8(private_key)
        .map_err(|e| FlowError::ValidationError(format!("Invalid private key: {}", e)))?;

    let key_pair = rcgen::KeyPair::from_pem(&key_pem)
        .map_err(|e| FlowError::ValidationError(format!("Invalid key pair: {}", e)))?;
    let _issuer = rcgen::Issuer::from_ca_cert_pem(&cert_pem, key_pair)
        .map_err(|e| FlowError::ValidationError(format!("Invalid CA certificate: {}", e)))?;

    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);
    crate::storage::ca::save_ca(&ca_path, &cert_pem, &key_pem).await;

    Ok(())
}
