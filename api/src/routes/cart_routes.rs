use axum::{routing::{get, post, put, delete}, Router};
use crate::controllers::cart::{
    get_cart, add_to_cart, delete_from_cart, update_cart
};
use sqlx::PgPool;

pub fn cart_routes() -> Router<PgPool> {
    Router::new()
        .route("/cart", get(get_cart))       
        .route("/cart/add", post(add_to_cart))
        .route("/cart/:id", put(update_cart).delete(delete_from_cart))
}