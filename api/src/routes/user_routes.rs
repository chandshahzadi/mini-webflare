use axum::{ 
    Router,
    routing::{get, put, post},
    middleware,
};
use sqlx::{PgPool}; 
use crate::controllers::user::{get_users, create_user, get_user, delete_user, update_user};
pub use mini_webflare::authentication::middleware::{auth_middleware};

pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .layer(middleware::from_fn(auth_middleware))
       
}