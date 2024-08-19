use crate::{app::AppContext, flows::FlowError};

pub async fn check_if_we_already_generated_ca(app: &AppContext) -> Result<(), FlowError> {
    let temp_dir = app.settings.get_temp_dir();

    let temp_vars_file = temp_dir.to_temp_vars_file();

    let result = tokio::fs::read(temp_vars_file.as_str()).await;

    if result.is_ok() {
        return Err(FlowError::CaAlreadyGenerated);
    }

    Ok(())
}
