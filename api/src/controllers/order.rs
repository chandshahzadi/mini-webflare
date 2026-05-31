use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use models::order::{CreateOrder, Order, UpdateOrder};
use utils::{db::DB, error::AppError};

// ➤ create/order
pub async fn create_order(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateOrder>,
) -> Result<(), AppError> {
    CreateOrder::create(db, payload).await
}

// ➤ get order
pub async fn get_order(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Order>>, AppError> {
    Order::get(db, user_id).await.map(Json)
}

// ➤ get all order
pub async fn get_orders(Extension(db): Extension<DB>) -> Result<Json<Vec<Order>>, AppError> {
    Order::find(db).await.map(Json)
}
// ➤ update order
pub async fn update_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, AppError> {
    payload.update(db, id).await.map(Json)
}

// ➤ delete/order
pub async fn delete_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Order::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/orders", get(get_orders).post(create_order))
        .route(
            "/orders/{id}",
            get(get_order).put(update_order).delete(delete_order),
        )
}
