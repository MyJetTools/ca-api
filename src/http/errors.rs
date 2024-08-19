use my_http_server::HttpFailResult;

use crate::flows::FlowError;

impl From<FlowError> for HttpFailResult {
    fn from(value: FlowError) -> Self {
        match value {
            FlowError::CaAlreadyGenerated => {
                Self::as_forbidden("Ca is already generated".to_string().into())
            }
            FlowError::CertNotFound => {
                Self::as_forbidden("Certification is not found".to_string().into())
            }
            FlowError::ValidationError(err) => Self::as_forbidden(err.into()),
            FlowError::SomethingWentWrong(err) => Self::as_forbidden(err.into()),
            FlowError::EasyRsaError(err) => Self::as_fatal_error(err.into()),
        }
    }
}
