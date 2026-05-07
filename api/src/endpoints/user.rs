use axum::{ 
    Extension, Json, Router, extract::Path, middleware, routing::{get, post, put}
};
use models::user::{CreateUser, User, UserRepo};
use sqlx::{PgPool}; 
use utils::{db::DB, error::AppError, middleware::auth_middleware};

// ➤ create/cart
pub async fn insert_user(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = UserRepo::insert_user(&db, &payload).await?;
    Ok(Json(user))
}

// ➤ get/product
pub async fn get_users(
     Extension(db): Extension<DB>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = UserRepo::get_user(&db).await?;
    Ok(Json(users))
}

// ➤ update/product 
pub async fn update_user(

    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let users = UserRepo::update_user(&db, id, &payload).await?;
    Ok(Json(users))
}

// ➤ delete/product
pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<String>, AppError> {
    UserRepo::delete_user(&db, id).await?;
    Ok(Json("Deleted successfully".to_string()))
}
pub fn router() -> Router<()> {
    Router::new()
        .route("/users", get(get_users).post(insert_user))
        .route("/users/:id", get(get_users).put(update_user).delete(delete_user))
        .layer(middleware::from_fn(auth_middleware))
       
}