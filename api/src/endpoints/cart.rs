use axum::{
    Json, Router, extract::{Extension, Path}, routing::{get, post}
};
use sqlx::PgPool;
use utils::{db::DB};
use models::cart::{CartRepo, CreateCart, UpdateCart, Cart};
use utils::error::AppError;

// ➤ create/cart
pub async fn add_to_cart(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, AppError> {
    CartRepo::insert_to_cart(&db, payload)
    .await
    .map(Json)
}

// ➤ get/cart
pub async fn get_cart(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Cart>>, AppError> {
    CartRepo::get_cart(&db, user_id)
    .await
    .map(Json)
}

// ➤ update/cart 
pub async fn update_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateCart>,
) -> Result<Json<Cart>, AppError> {
    CartRepo::update_cart(&db, id, payload.quantity)
    .await
    .map(Json)
}

// ➤ delete/cart
pub async fn delete_from_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<String>, AppError> {
     CartRepo::delete_from_cart(&db, id)
    .await
    .map(Json("Deleted successfully".to_string()))
}

pub fn router() -> Router<PgPool> {    
    Router::new()
        .route("/cart/add", post(add_to_cart))
        .route("/cart", get(get_cart))
        .route(
            "/cart/:id",
            get(get_cart).put(update_cart).delete(delete_from_cart)
        )
}
