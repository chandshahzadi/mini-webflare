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
    let cart = CartRepo::insert_to_cart(&db, payload).await?;
    Ok(Json(cart))
}

// ➤ get/cart
pub async fn get_cart(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Cart>>, AppError> {
    let carts = CartRepo::get_cart(&db, user_id).await?;
    Ok(Json(carts))
}

// ➤ update/cart 
pub async fn update_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateCart>,
) -> Result<Json<Cart>, AppError> {
    let cart = CartRepo::update_cart(&db, id, payload.quantity).await?;
    Ok(Json(cart))
}

// ➤ delete/cart
pub async fn delete_from_cart(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<String>, AppError> {
    CartRepo::delete_from_cart(&db, id).await?;
    Ok(Json("Deleted successfully".to_string()))
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
