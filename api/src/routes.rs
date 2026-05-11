use axum::{
    Extension, Router}
;
use utils::{db::DB};
use crate::{authentication, controllers};

pub fn router(state: DB) -> Router {
    let routes = Router::new()
        .nest("/auth", authentication::router())
        .nest("/controllers", controllers::router());
}