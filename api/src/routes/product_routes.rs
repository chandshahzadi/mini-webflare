use axum::{routing::{get, post}, Router};
pub use crate::controllers::product::{get_products, create_product, update_product, delete_product};
use sqlx::PgPool;

pub fn product_routes() -> Router<PgPool> {
    Router::new()
        .route("/products", get(get_products).post(create_product))
        .route("/products/:id", get(create_product).put(update_product).delete(delete_product))
}
