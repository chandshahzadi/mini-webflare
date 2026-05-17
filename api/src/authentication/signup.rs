use axum::{
    extract::{State}, Json, Router, routing::post
};
use utils::{db::DB, error::AppError};
use authentication::signup::{SignupInput, signup};

pub async fn sign_up(
    State(db): State<DB>,
    Json(form): Json<SignupInput>,
) -> Result<Json<String>, AppError> {
    match signup(db, form).await {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(AppError::DbError("signup failed".to_owned())),
    }
}

pub fn router() -> Router<DB>{
    Router::new().route("/signup", post(sign_up))
}