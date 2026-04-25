use crate::app::AppContext;
use crate::flows::FlowError;

pub async fn check_if_we_have_ca_cert(app: &AppContext, ca_cn: &str) -> Result<(), FlowError> {
    let ca_path = app.settings.get_config_path().into_ca_data_path(ca_cn);

    if tokio::fs::metadata(ca_path.to_ca_cert_file_name())
        .await
        .is_ok()
    {
        return Ok(());
    }

    Err(FlowError::CaNotFound)
}
