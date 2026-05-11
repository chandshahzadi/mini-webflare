use axum::{ 
    Extension, Json, Router, extract::Path, middleware, routing::{get, post, put}
};
use utils::{db::DB, error::AppError, middleware::auth_middleware};
use models::user::{CreateUser, User};

// ➤ create/cart
pub async fn create(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = User::create(&db, &payload).await?;
    Ok(Json(user))
}

// ➤ get/product
pub async fn get(
     Extension(db): Extension<DB>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = User::get(&db).await?;
    Ok(Json(users))
}

// ➤ update/product 
pub async fn update(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let users = User::update(&db, id, &payload).await?;
    Ok(Json(users))
}

// ➤ delete/product
pub async fn delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    User::delete(&db, id).await?;
    Ok(())
}
pub fn router() -> Router<()> {
    Router::new()
        .route("/users", get(get).post(create))
        .route("/users/:id", get(get).put(update).delete(delete))
        .layer(middleware::from_fn(auth_middleware))
}