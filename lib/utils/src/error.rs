use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum AppError {
    DbError(String),
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::DbError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = match self {
            AppError::DbError(msg) => msg,
        };

        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}