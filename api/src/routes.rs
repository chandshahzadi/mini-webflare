use crate::authentication;
use crate::controllers;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/authentication", authentication::router())
        .nest("/controllers", controllers::router())
}
