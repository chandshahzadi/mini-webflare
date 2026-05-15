use axum::{
    Json, Router, extract::Extension, routing::post
};
use utils::{db::DB, error::AppError};
use authentication::signup::{SignupInput, signup};

pub async fn sign_up(
    Extension(db): Extension<DB>,
    Json(form): Json<SignupInput>,
) -> Result<Json<String>, AppError> {
    Ok(signup(axum::extract::State(db), Json(form)).await)
}

pub fn router() -> Router<DB>{
    Router::new().route("/signup", post(sign_up))
}