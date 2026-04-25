use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;
use crate::flows::FlowError;

#[http_route(
    method: "GET",
    route: "/api/ca/v1/downloadCert",
    summary: "Download CA certificate file",
    description: "Download CA certificate file",
    controller: "Certificate Authority",
    input_data: "DownloadCaCertInputModel",
    result:[
        {status_code: 200, description: "CA certificate as PEM text"},
    ]
)]
pub struct DownloadCertAction {
    app: Arc<AppContext>,
}

impl DownloadCertAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadCertAction,
    input_data: DownloadCaCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let ca_path = action
        .app
        .settings
        .get_config_path()
        .into_ca_data_path(&input_data.ca_name);

    match tokio::fs::read_to_string(ca_path.to_ca_cert_file_name()).await {
        Ok(content) => HttpOutput::as_text(content).into_ok_result(true).into(),
        Err(_) => Err(FlowError::CaNotFound.into()),
    }
}

#[derive(MyHttpInput)]
struct DownloadCaCertInputModel {
    #[http_query(name = "caName", description = "CA Common Name")]
    pub ca_name: String,
}
