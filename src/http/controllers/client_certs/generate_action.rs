use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/certificates/v1/generate",
    summary: "Generate Client Certificate",
    description: "Generate Client Certificate",
    controller: "Client Certificates",
    input_data: "GenerateCertificateInputModel",

    result:[
        {status_code: 202, description: "Generated Certificate"},
    ]
)]
pub struct GenerateCertificateAction {
    app: Arc<AppContext>,
}

impl GenerateCertificateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GenerateCertificateAction,
    input_data: GenerateCertificateInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::generate_cert(&action.app, &input_data.email).await?;
    HttpOutput::Empty.into_ok_result(true).into()
}

#[derive(MyHttpInput)]
pub struct GenerateCertificateInputModel {
    #[http_body(name = "email", description = "Email")]
    pub email: String,
}
