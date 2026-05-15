use axum::{ 
    Extension, Json, Router, extract::Path, middleware, routing::{get, post, put}
};
use utils::{db::DB, error::AppError, middleware::auth_middleware};
use models::user::{CreateUser, User};

// ➤ create/cart
pub async fn create_user(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<(), AppError> {
     CreateUser::create(db, &payload).await
}

// ➤ get/product
pub async fn get_user(
     Extension(db): Extension<DB>,
) -> Result<Json<Vec<User>>, AppError> {
     User::get(db).await.map(Json)
}

// ➤ update/product 
pub async fn update_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
     User::update(db, id, &payload).await.map(Json)
}

// ➤ delete/product
pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    User::delete(db, id).await
}
pub fn router() -> Router<DB> {
    Router::new()
        .route("/users", get(get_user).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .layer(middleware::from_fn(auth_middleware))
}