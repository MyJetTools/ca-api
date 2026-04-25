use my_http_server::HttpFailResult;

use crate::flows::FlowError;

impl From<FlowError> for HttpFailResult {
    fn from(value: FlowError) -> Self {
        match value {
            FlowError::CaAlreadyGenerated => {
                Self::as_forbidden("CA is already generated".to_string().into())
            }
            FlowError::CaNotFound => Self::as_not_found("CA not found".to_string(), false),
            FlowError::CertNotFound => Self::as_not_found("Certificate not found".to_string(), false),
            FlowError::ValidationError(err) => Self::as_forbidden(err.into()),
            FlowError::SomethingWentWrong(err) => Self::as_fatal_error(err),
        }
    }
}
