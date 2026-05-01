use axum::{Router, routing::post};
use crate::controllers::auth::{signup, login};
use sqlx::PgPool;

// aunthentication routing
pub fn auth_route() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}