use authentication::signup::Signup;
use axum::{Extension, Json, Router, routing::post};
use utils::{db::DB, error::AppError};

pub async fn sign_up(
    Extension(db): Extension<DB>,
    Json(form): Json<Signup>,
) -> Result<(), AppError> {
    form.signup(db).await
}

pub fn router() -> Router {
    Router::new().route("/signup", post(sign_up))
}
