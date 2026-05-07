use authentication::login::LoginInput;
use axum::{Json, Router, extract::Extension, routing::post};
use utils::{db::DB, error::AppError};

pub async fn login(
    Extension(db): Extension<DB>,
    Json(form): Json<LoginInput>,
) -> Result<Json<String>, AppError> {
    Ok(LoginInput::login(Json(form)).await)
}

pub fn router() -> Router {
    Router::new().route("/login", post(login))
}