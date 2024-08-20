use std::sync::Arc;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_crl(app: &Arc<AppContext>) -> Result<String, FlowError> {
    let easy_rsa_command = app.get_easy_rsa_command();

    let result = tokio::process::Command::new(easy_rsa_command.as_str())
        .arg("gen-crl")
        .output()
        .await
        .unwrap();

    FlowError::check_error(&result)?;

    let crl = app.get_crl_file();

    match tokio::fs::read_to_string(crl.as_str()).await {
        Ok(crl) => return Ok(crl),
        Err(e) => {
            return Err(FlowError::SomethingWentWrong(format!(
                "Failed to read CRL file {}. Err: {:?}",
                crl.as_str(),
                e,
            )))
        }
    }
}
