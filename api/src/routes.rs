use axum::{Extension, Router};
use utils::{
    db::DB
};
use crate::{authentication, endpoints};

pub fn router(state: DB) -> Router {
    let routes = Router::new()
        .nest("/auth", authentication::router())
        .nest("/routing", endpoints::router());
    
    Router::new()
        .merge(authentication::router())
        .merge(endpoints::router())
        .layer(Extension(state))
}