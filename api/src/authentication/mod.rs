use axum::Router;
pub mod forgot_pass;
pub mod login;
pub mod signup;

pub fn router() -> Router {
    Router::new()
        .merge(login::router())
        .merge(signup::router())
        .merge(forgot_pass::router())
}
