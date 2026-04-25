use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use serde::*;

use crate::app::AppContext;
use crate::flows::IssuedCertificateInfo;

#[http_route(
    method: "GET",
    route: "/api/certificates/v1/list",
    summary: "Get list of certificates",
    description: "Get list of certificates",
    controller: "Client Certificates",
    input_data: "GetListInputModel",
    result:[
        {status_code: 200, description: "List of issued certificates"},
    ]
)]
pub struct GetListOfCertificatesAction {
    app: Arc<AppContext>,
}

impl GetListOfCertificatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetListOfCertificatesAction,
    input_data: GetListInputModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_list_of_issued_certificates(&action.app, &input_data.ca_name).await;

    let response: Vec<IssuedCertificateHttpModel> = result.into_iter().map(|x| x.into()).collect();

    return HttpOutput::as_json(response).into_ok_result(true).into();
}

#[derive(MyHttpInput)]
struct GetListInputModel {
    #[http_query(name = "caName", description = "CA Common Name")]
    pub ca_name: String,
}

#[derive(MyHttpInputObjectStructure, Serialize, Deserialize)]
struct IssuedCertificateHttpModel {
    pub cn: String,
    pub revoked: bool,
}

impl Into<IssuedCertificateHttpModel> for IssuedCertificateInfo {
    fn into(self) -> IssuedCertificateHttpModel {
        IssuedCertificateHttpModel {
            cn: self.cn,
            revoked: self.revoked,
        }
    }
}
