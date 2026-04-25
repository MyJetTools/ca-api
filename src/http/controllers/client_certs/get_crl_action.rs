use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/revoked/v1/crl",
    summary: "Get CRL",
    description: "Get CRL",
    controller: "Client Certificates",
    input_data: "GetCrlInputModel",
    result:[
        {status_code: 200, description: "CRL PEM", model: "String"},
    ]
)]
pub struct GetCrlAction {
    app: Arc<AppContext>,
}

impl GetCrlAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetCrlAction,
    input_data: GetCrlInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let content = crate::flows::get_crl(&action.app, &input_data.ca_name).await?;
    return HttpOutput::as_text(content).into_ok_result(true);
}

#[derive(MyHttpInput)]
struct GetCrlInputModel {
    #[http_query(name = "caName", description = "CA Common Name")]
    pub ca_name: String,
}
