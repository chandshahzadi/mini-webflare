use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;
#[derive(Debug)]
pub enum AppError {
    DbError(String),
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::DbError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DbError(msg) => {
                eprintln!("database error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };

        (status, message).into_response()
    }
}
