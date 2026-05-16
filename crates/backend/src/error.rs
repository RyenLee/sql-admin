use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sql_admin_shared::AppError;

#[allow(dead_code)]
pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[allow(dead_code)]
pub struct AppErrorResponse(pub AppError);

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConnectionNotFound => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::QueryFailed(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match self.0 {
            AppError::Database(_) => "Database error".to_string(),
            AppError::ConnectionNotFound => "Connection not found".to_string(),
            AppError::Validation(msg) => msg,
            AppError::QueryFailed(msg) => msg,
            AppError::InternalError => "Internal server error".to_string(),
        };

        (status, error_message).into_response()
    }
}
