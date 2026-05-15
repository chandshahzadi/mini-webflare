use axum::Router;
use utils::db::DB;
use crate::authentication;
use crate::controllers;

pub fn router() -> Router<DB> {
    Router::new()
        .nest("/authentication", authentication::router())
        .nest("/controllers", controllers::router())
}