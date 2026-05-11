use axum::{
    Json, Router, extract::{Extension, Path}, routing::{get, post}
};
use utils::{db::DB, error::AppError};
use models::cart::{Cart, CreateCart, UpdateCart, Cart};

// ➤ create/cart
pub async fn add_to_cart(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, AppError> {
    Cart::create(&db, payload)
    .await
    .map(Json)
}

// ➤ get/cart
pub async fn get_cart(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Cart>>, AppError> {
    Cart::get(&db, user_id)
    .await
    .map(Json)
}

// ➤ update/cart 
pub async fn update_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateCart>,
) -> Result<Json<Cart>, AppError> {
    Cart::update(&db, id, payload.quantity)
    .await
    .map(Json)
}

// ➤ delete/cart
pub async fn delete_from_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Cart::delete(&db, id).await
}

pub fn router() -> Router<()> {    
    Router::new()
        .route("/cart/add", post(add_to_cart))
        .route("/cart", get(get_cart))
        .route(
            "/cart/:id",
            get(get_cart).put(update_cart).delete(delete_from_cart)
        )
}
