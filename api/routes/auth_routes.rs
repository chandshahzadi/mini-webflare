use axum::{Router, routing::post};
use mini_webflare::handlers::auth_handler::{signup, login};
use sqlx::PgPool;

// aunthentication routing
pub fn auth_route() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}