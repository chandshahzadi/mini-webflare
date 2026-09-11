use authentication::forgot_pass::ForgotPassword;
use axum::{Extension, Json, Router, routing::post};
use utils::{db::DB, error::AppError};

pub async fn forgot_pass(
    Extension(db): Extension<DB>,
    Json(form): Json<ForgotPassword>,
) -> Result<(), AppError> {
    form.request(db).await
}

pub fn router() -> Router {
    Router::new().route("/forgot-password", post(forgot_pass))
}
