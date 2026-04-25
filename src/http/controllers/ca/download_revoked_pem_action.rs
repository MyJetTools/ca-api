use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/ca/v1/downloadRevokedPem",
    summary: "Download per-CA CRL",
    description: "Download per-CA CRL",
    controller: "Certificate Authority",
    input_data: "DownloadRevokedInputModel",
    result:[
        {status_code: 200, description: "CRL as PEM text"},
    ]
)]
pub struct DownloadRevokedAction {
    app: Arc<AppContext>,
}

impl DownloadRevokedAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &DownloadRevokedAction,
    input_data: DownloadRevokedInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content = crate::flows::get_crl(&action.app, &input_data.ca_name).await?;
    return HttpOutput::as_text(content).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct DownloadRevokedInputModel {
    #[http_query(name = "caName", description = "CA Common Name")]
    pub ca_name: String,
}
