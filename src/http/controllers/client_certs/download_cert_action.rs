use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/certificates/v1/downloadCert",
    summary: "Download pfx",
    description: "Download pfx",
    controller: "Client Certificates",
    input_data: "DownloadClientCertInputModel",
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
    input_data: DownloadClientCertInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result =
        crate::flows::get_p12(&action.app, &input_data.email, &input_data.password).await?;

    let file_name = format!("{}.p12", input_data.email);

    return HttpOutput::as_file(file_name, result)
        .into_ok_result(true)
        .into();
}

#[derive(MyHttpInput)]
struct DownloadClientCertInputModel {
    #[http_query(name = "email", description = "Email")]
    pub email: String,
    #[http_query(name = "password", description = "Certificate Password")]
    pub password: String,
}
