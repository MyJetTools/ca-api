use std::sync::Arc;

use crate::{app::AppContext, flows::FlowError};

pub use crate::pem::PemCertInfo;

pub async fn generate_ca(app: &Arc<AppContext>, cert_info: PemCertInfo) -> Result<(), FlowError> {
    crate::scripts::check_if_we_already_generated_ca(app, &cert_info.ca_cn).await?;

    let generated = crate::crypto::build_ca(&cert_info).map_err(FlowError::SomethingWentWrong)?;

    let ca_path = app
        .settings
        .get_config_path()
        .into_ca_data_path(&cert_info.ca_cn);

    crate::storage::ca::save_ca(&ca_path, &generated.cert_pem, &generated.key_pem).await;

    Ok(())
}
