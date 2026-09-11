use axum::{
    Extension, Json, Router,
    extract::Path,
    middleware::from_fn,
    routing::{get, put},
};
use models::order::{CreateOrder, Order, UpdateOrder};
use utils::middleware::{AuthUser, is_admin};
use utils::{db::DB, error::AppError};

// ➤ create order for the login user
pub async fn create_order(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, AppError> {
    payload.create(db, user.id).await.map(Json)
}

// ➤ get the login user's own orders
pub async fn get_my_orders(
    Extension(db): Extension<DB>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Vec<Order>>, AppError> {
    Order::find_by_user_id(db, user.id).await.map(Json)
}

// ➤ get all orders admin
pub async fn get_orders(Extension(db): Extension<DB>) -> Result<Json<Vec<Order>>, AppError> {
    Order::find(db).await.map(Json)
}

// ➤ get orders of a specific user admin
pub async fn get_user_orders(
    Path(user_id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Order>>, AppError> {
    Order::find_by_user_id(db, user_id).await.map(Json)
}

// ➤ update order admin
pub async fn update_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, AppError> {
    payload.update(db, id).await.map(Json)
}

// ➤ delete order admin
pub async fn delete_order(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Order::delete(db, id).await
}

pub fn router() -> Router {
    let admin = Router::new()
        .route("/orders/all", get(get_orders))
        .route("/orders/user/{user_id}", get(get_user_orders))
        .route("/orders/{id}", put(update_order).delete(delete_order))
        .layer(from_fn(is_admin));

    Router::new()
        .route("/orders", get(get_my_orders).post(create_order))
        .merge(admin)
}
