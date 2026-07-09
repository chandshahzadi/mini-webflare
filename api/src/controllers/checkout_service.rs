use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use models::checkout_service::CheckoutRequest;
use utils::{db::DB, error::AppError};

pub async fn checkout_handler(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CheckoutRequest>,
) -> Result<Json<i32>, AppError> {
    println!("user_id = {:?}", user_id);
    payload.checkout(db, user_id).await.map(Json)
}

pub fn router() -> Router {
    Router::new().route("/checkout/{user_id}", post(checkout_handler))
}
