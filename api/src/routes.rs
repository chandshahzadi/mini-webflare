use crate::authentication;
use crate::controllers;
use axum::{Router, middleware::from_fn};
use utils::middleware::verify_token;

pub fn router() -> Router {
    Router::new()
        .nest("/authentication", authentication::router())
        .merge(controllers::router().layer(from_fn(verify_token)))
}
