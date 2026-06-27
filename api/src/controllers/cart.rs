use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post, put},
};
use models::cart::{Cart, CartItem, CartResult, CreateCart, CreateCartItem};
use utils::{db::DB, error::AppError};

pub async fn create_cart(
    Extension(db): Extension<DB>,
    Json(form): Json<CreateCart>,
) -> Result<Json<Cart>, AppError> {
    form.create_cart(db).await.map(Json)
}

// ➤ create cart
pub async fn add_to_cart(
    Extension(db): Extension<DB>,
    Json(form): Json<CreateCartItem>,
) -> Result<(), AppError> {
    form.create_cart_item(db).await
}

// ➤ get user_id
pub async fn get_cart(
    Extension(db): Extension<DB>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<CartResult>>, AppError> {
    println!("user_id = {}", id);
    CartResult::find_by_user_id(db, id).await.map(Json)
}

// ➤ get cart id
pub async fn get_cart_by_id(
    Extension(db): Extension<DB>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<CartResult>>, AppError> {
    CartResult::find_by_cart_id(db, id).await.map(Json)
}

// ➤ update carts
pub async fn update_cart_items(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CartItem>,
) -> Result<Json<CartItem>, AppError> {
    CartItem::update(db, id, payload.quantity).await.map(Json)
}

// // ➤ delete cart
pub async fn delete_cart_items(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    CartItem::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/cart", post(create_cart))
        .route("/cart/add", post(add_to_cart))
        .route("/cart/user/{id}", get(get_cart))
        .route(
            "/cart/{id}",
            get(get_cart_by_id)
                .put(update_cart_items)
                .delete(delete_cart_items),
        )
}
