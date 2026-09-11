use axum::{Extension, Json, Router, routing::post};
use models::checkout_service::CheckoutRequest;
use utils::{db::DB, error::AppError, middleware::AuthUser};

// ➤ checkout the logged-in user's cart
pub async fn checkout_handler(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CheckoutRequest>,
) -> Result<Json<i32>, AppError> {
    payload.checkout(db, user.id).await.map(Json)
}

pub fn router() -> Router {
    Router::new().route("/checkout", post(checkout_handler))
}
