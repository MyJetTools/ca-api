use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/ca/v1/downloadCert",
    summary: "Download certificate file",
    description: "Download certificate file",
    controller: "Certificate Authority",
    result:[
        {status_code: 200, description: "Certificate as a text"},
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
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    match tokio::fs::read_to_string(action.app.get_ca_cert_file()).await {
        Ok(content) => HttpOutput::as_text(content).into_ok_result(true).into(),
        Err(err) => Err(HttpFailResult::as_not_found(
            format!("Failed to read certificate file. Err: {:?}", err),
            false,
        )),
    }
}
