use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post, put},
};
use models::cart::{Cart, CartItem, CartResult, CreateCartItem};
use utils::{db::DB, error::AppError, middleware::AuthUser};

pub async fn create_cart(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Cart>, AppError> {
    Cart::get_or_create(&db, user.id).await.map(Json)
}

// ➤ create cart
pub async fn add_to_cart(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
    Json(form): Json<CreateCartItem>,
) -> Result<(), AppError> {
    form.create_cart_item(db, user.id).await
}

// ➤ get the logged-in user's own cart
pub async fn get_cart(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Vec<CartResult>>, AppError> {
    CartResult::find_by_user_id(db, user.id).await.map(Json)
}

// ➤ update carts
pub async fn update_cart_items(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CartItem>,
) -> Result<Json<CartItem>, AppError> {
    CartItem::update(db, id, user.id, payload.quantity)
        .await
        .map(Json)
}

// // ➤ delete cart
pub async fn delete_cart_items(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
) -> Result<(), AppError> {
    CartItem::delete(db, id, user.id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/cart", get(get_cart).post(create_cart))
        .route("/cart/add", post(add_to_cart))
        .route(
            "/cart/{id}",
            put(update_cart_items).delete(delete_cart_items),
        )
}
