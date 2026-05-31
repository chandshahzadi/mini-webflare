use axum::Router;
pub mod login;
pub mod signup;

pub fn router() -> Router {
    Router::new().merge(login::router()).merge(signup::router())
}
