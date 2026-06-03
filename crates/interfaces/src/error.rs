use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sql_admin_domain::shared::application_error::ApplicationError;

#[derive(Debug)]
pub struct AppErrorResponse(pub ApplicationError);

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            ApplicationError::Domain(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            ApplicationError::Infrastructure(_) => {
                tracing::error!(error = %self.0, "Infrastructure error");
                (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())
            }
            ApplicationError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApplicationError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApplicationError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        (status, message).into_response()
    }
}

impl From<ApplicationError> for AppErrorResponse {
    fn from(err: ApplicationError) -> Self {
        Self(err)
    }
}

pub type AppResult<T> = std::result::Result<T, AppErrorResponse>;