pub mod authentication;
pub mod controllers;
pub mod routes;

use axum::{Extension, Router};
use utils::db::DB;

pub fn create_app(db: DB) -> Router {
    routes::router().layer(Extension(db))
}
