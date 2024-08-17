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
    input_data: "GetRevokedCertsInputModel",
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
    input_data: GetRevokedCertsInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let path = action
        .app
        .settings
        .get_config_path()
        .into_ca_data_path(&input_data.ca_name);

    let crl_content = tokio::fs::read(path.to_crl_file_name()).await;

    match crl_content {
        Ok(crl_content) => {
            return HttpOutput::as_text(String::from_utf8(crl_content).unwrap())
                .into_ok_result(true)
                .into();
        }
        Err(_) => {
            return HttpOutput::as_text(my_tls::crl::EMPTY_CRL.to_string())
                .into_ok_result(true)
                .into();
        }
    }
}

#[derive(MyHttpInput)]
struct GetRevokedCertsInputModel {
    #[http_query(name = "caName", description = "Common name")]
    pub ca_name: String,
}
