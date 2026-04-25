use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;
use crate::flows::FlowError;

#[http_route(
    method: "GET",
    route: "/api/ca/v1/downloadPk",
    summary: "Download CA private key",
    description: "Download CA private key",
    controller: "Certificate Authority",
    input_data: "DownloadCaPrivateKeyInputModel",
    result:[
        {status_code: 200, description: "CA private key as PEM text"},
    ]
)]
pub struct DownloadPrivateKeyAction {
    app: Arc<AppContext>,
}

impl DownloadPrivateKeyAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadPrivateKeyAction,
    input_data: DownloadCaPrivateKeyInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let ca_path = action
        .app
        .settings
        .get_config_path()
        .into_ca_data_path(&input_data.ca_name);

    match tokio::fs::read_to_string(ca_path.to_ca_private_key_file_name()).await {
        Ok(content) => HttpOutput::as_text(content).into_ok_result(true).into(),
        Err(_) => Err(FlowError::CaNotFound.into()),
    }
}

#[derive(MyHttpInput)]
struct DownloadCaPrivateKeyInputModel {
    #[http_query(name = "caName", description = "CA Common Name")]
    pub ca_name: String,
}
