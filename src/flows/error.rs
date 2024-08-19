use std::process::Output;

#[derive(Debug)]
pub enum FlowError {
    CaAlreadyGenerated,
    CertNotFound,
    ValidationError(String),
    SomethingWentWrong(String),
    EasyRsaError(String),
}

impl FlowError {
    pub fn check_error(result: &Output) -> Result<(), FlowError> {
        if !result.status.success() {
            return Err(FlowError::EasyRsaError(format!("{:#?}", result)));
        }
        Ok(())
    }
}
