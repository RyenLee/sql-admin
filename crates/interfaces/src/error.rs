use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sql_admin_api_types::ApiResponse;
use sql_admin_domain::shared::application_error::ApplicationError;

#[derive(Debug)]
pub struct AppErrorResponse(pub ApplicationError);

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            ApplicationError::Domain(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            ApplicationError::Infrastructure(_) => {
                tracing::error!(error = %self.0, "Infrastructure error");
                // Don't expose internal details to clients
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            ApplicationError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApplicationError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApplicationError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        let body = ApiResponse::<()>::err(&message);
        (status, Json(body)).into_response()
    }
}

impl From<ApplicationError> for AppErrorResponse {
    fn from(err: ApplicationError) -> Self {
        Self(err)
    }
}

pub type AppResult<T> = std::result::Result<T, AppErrorResponse>;