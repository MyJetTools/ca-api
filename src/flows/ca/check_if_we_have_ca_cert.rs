use crate::{app::AppContext, flows::FlowError, pem::PemCertificate};

pub async fn check_if_we_have_ca_cert(app: &AppContext, ca_cn: &str) -> Result<(), FlowError> {
    let ca_cert_file = app.settings.get_config_path().into_ca_data_path(ca_cn);

    let cert_file_name = ca_cert_file.to_cert_file_name();

    let content = tokio::fs::read(cert_file_name.as_str()).await;

    if content.is_err() {
        println!("Can not read file: {:?}", cert_file_name);
        return Err(FlowError::CaNotFound);
    }

    let content = content.unwrap();

    let pem = PemCertificate::from_bytes(content);

    let info = pem.get_cert_info();

    if info.is_err() {
        println!("Can not read CA info from file: {:?}", cert_file_name);
        return Err(FlowError::CaNotFound);
    }

    Ok(())
}
