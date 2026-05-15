use axum::{ 
    Extension, Json, Router, extract::Path, middleware, routing::{get, post, put}
};
use utils::{db::DB, error::AppError, middleware::auth_middleware};
use models::user::{CreateUser, User};

// ➤ create/cart
pub async fn create(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<()>, AppError> {
     User::create(&db, &payload).await
}

// ➤ get/product
pub async fn get(
     Extension(db): Extension<DB>,
) -> Result<Json<Vec<User>>, AppError> {
     User::get(&db).await.map(Json)
}

// ➤ update/product 
pub async fn update(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
     User::update(&db, id, &payload).await.map(Json)
}

// ➤ delete/product
pub async fn delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    User::delete(&db, id).await
}
pub fn router() -> Router<DB> {
    Router::new()
        .route("/users", get(get).post(create))
        .route("/users/:id", get(get).put(update).delete(delete))
        .layer(middleware::from_fn(auth_middleware))
}