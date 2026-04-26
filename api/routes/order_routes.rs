use axum::{routing::{get, post, delete}, Router};
pub use crate::handlers::order_handler::{create_order, get_orders, update_order, delete_order};
use sqlx::PgPool;

pub fn order_routes() -> Router<PgPool> {
    Router::new()
        .route("/orders", get(get_orders).post(create_order))
        .route("/orders/:id", get(get_orders).put(update_order).delete(delete_order))
}

