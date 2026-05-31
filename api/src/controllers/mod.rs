use axum::Router;
pub mod cart;
pub mod order;
pub mod product;
pub mod user;

pub fn router() -> Router {
    Router::new()
        .merge(user::router())
        .merge(product::router())
        .merge(cart::router())
        .merge(order::router())
}
