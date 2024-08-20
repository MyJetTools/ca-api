use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/revoked/v1/crl",
    summary: "Get List of revoked certificates",
    description: "Get List of revoked certificates",
    controller: "Client Certificates",
    result:[
        {status_code: 200, description: "List of revoked certificates", model: "String"},

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
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::get_crl(&action.app).await?;
    return HttpOutput::Empty.into_ok_result(true).into();
}
