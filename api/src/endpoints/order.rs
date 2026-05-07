use axum::{Extension, Json, Router, extract::Path, routing::{delete, get, post}};
use models::order::{CreateOrder, Order, OrderRepo};
use utils::db::DB;
use sqlx::PgPool;
use utils::error::AppError;

// ➤ create/order
pub async fn create_order(

    Extension(db): Extension<DB>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, AppError> {
    let order = OrderRepo::create_order(&db, payload).await?;
    Ok(Json(order))
}

// ➤ get/order
pub async fn get_orders(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Order>>, AppError> {
    let orders = OrderRepo::get_orders(&db, user_id).await?;
    Ok(Json(orders))
}

// ➤ update/order 
pub async fn update_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, AppError> {
    let order = OrderRepo::update_order(&db, id, payload.quantity).await?;
    Ok(Json(order))
}

// ➤ delete/order
pub async fn delete_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<String>, AppError> {
    OrderRepo::delete_order(&db, id).await?;
    Ok(Json("Deleted successfully".to_string()))
}
pub fn router() -> Router<()> {
    Router::new()
        .route("/orders", get(get_orders).post(create_order))
        .route("/orders/:id", get(get_orders).put(update_order).delete(delete_order))
}

