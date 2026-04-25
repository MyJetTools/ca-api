#[derive(Debug)]
pub enum FlowError {
    CaAlreadyGenerated,
    CaNotFound,
    CertNotFound,
    ValidationError(String),
    SomethingWentWrong(String),
}
