use axum::{ 
    Extension, Json, Router, extract::Path, middleware, routing::{get, post, put}
};
use models::user::{CreateUser, User, UserRepo};
use sqlx::{PgPool}; 
use utils::{db::DB, error::AppError, middleware::auth_middleware};

// ➤ create/cart
pub async fn create(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = UserRepo::create(&db, &payload).await?;
    Ok(Json(user))
}

// ➤ get/product
pub async fn get(
     Extension(db): Extension<DB>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = UserRepo::get(&db).await?;
    Ok(Json(users))
}

// ➤ update/product 
pub async fn update(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let users = UserRepo::update(&db, id, &payload).await?;
    Ok(Json(users))
}

// ➤ delete/product
pub async fn delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<String>, AppError> {
    UserRepo::delete(&db, id).await?;
    Ok(Json("Deleted successfully".to_string()))
}
pub fn router() -> Router<()> {
    Router::new()
        .route("/users", get(get).post(create))
        .route("/users/:id", get(get).put(update).delete(delete))
        .layer(middleware::from_fn(auth_middleware))
       
}