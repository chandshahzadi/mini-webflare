use axum::{
    extract::Extension, Json, Router, routing::post
};
use utils::{db::DB, error::AppError};
use authentication::login::LoginInput;

pub async fn login(
    Extension(db): Extension<DB>,
    Json(form): Json<LoginInput>,
) -> Result<Json<String>, AppError> {
    Ok(LoginInput::login(Json(form)).await)
}

pub fn router() -> Router<DB> {
    Router::new().route("/login", post(login))
}