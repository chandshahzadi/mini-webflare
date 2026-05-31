use authentication::signup::{SignupInput, signup};
use axum::{Extension, Json, Router, routing::post};
use utils::{db::DB, error::AppError};

pub async fn sign_up(
    Extension(db): Extension<DB>,
    Json(form): Json<SignupInput>,
) -> Result<Json<String>, AppError> {
    match signup(db, form).await {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(AppError::DbError("signup failed".to_owned())),
    }
}

pub fn router() -> Router {
    Router::new().route("/signup", post(sign_up))
}
