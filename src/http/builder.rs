use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    //result.register_get_action(Arc::new(
    //    super::controllers::client_certs::GetListOfRevokedCertificatesAction::new(app.clone()),
    //));

    result.register_post_action(Arc::new(
        crate::http::controllers::client_certs::GenerateCertificateAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::client_certs::RevokeCertificateAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::DownloadCertAction::new(app.clone()),
    ));

    //result.register_get_action(Arc::new(
    //    crate::http::controllers::client_certs::DownloadPemCertificateAction::new(app.clone()),
    //));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::GetCrlAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::GetListOfCertificatesAction::new(app.clone()),
    ));

    //CA Controller

    //    result.register_post_action(Arc::new(crate::http::controllers::ca::CheckCaAction::new(
    //        app.clone(),
    //    )));

    result.register_post_action(Arc::new(
        crate::http::controllers::ca::GenerateCaAction::new(app.clone()),
    ));

    //    result.register_post_action(Arc::new(crate::http::controllers::ca::ImportCaAction::new(
    //        app.clone(),
    //    )));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadCertAction::new(app.clone()),
    ));

    //    result.register_get_action(Arc::new(
    //        crate::http::controllers::ca::DownloadRevokedAction::new(app.clone()),
    //    ));

    //    result.register_get_action(Arc::new(
    //        crate::http::controllers::ca::DownloadPrivateKeyAction::new(app.clone()),
    //    ));

    //    result.register_get_action(Arc::new(
    //        crate::http::controllers::ca::GetListOfCaAction::new(app.clone()),
    //    ));

    result
}
