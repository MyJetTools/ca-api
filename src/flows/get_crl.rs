use std::sync::Arc;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_crl(app: &Arc<AppContext>) -> Result<Vec<u8>, FlowError> {
    let easy_rsa_command = app.get_easy_rsa_command();

    let result = tokio::process::Command::new(easy_rsa_command.as_str())
        .arg("gen-crl")
        .output()
        .await
        .unwrap();

    FlowError::check_error(&result)?;

    Ok(vec![])
}
