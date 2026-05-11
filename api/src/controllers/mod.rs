use axum::Router;
pub mod user;
pub mod product;
pub mod cart;
pub mod order;

pub fn router() -> Router<()> {
    Router::new()
        .merge(user::router())
        .merge(product::router())
        .merge(cart::router())
        .merge(order::router())
}