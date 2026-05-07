use axum::Router;
pub mod signup;
pub mod login;

pub fn router() -> Router {
    Router::new()
        .merge(login::router())
        .merge(signup::router())
}