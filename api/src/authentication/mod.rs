use axum::Router;
use utils::db::DB;
pub mod signup;
pub mod login;

pub fn router() -> Router<DB> {
    Router::new()
        .merge(login::router())
        .merge(signup::router())
}