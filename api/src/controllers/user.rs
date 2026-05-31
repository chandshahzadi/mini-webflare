use crate::authentication;
use crate::controllers;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{get, post, put},
};
use models::user::{CreateUser, User};
use utils::{db::DB, error::AppError};

// ➤ create user
pub async fn create_user(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<(), AppError> {
    CreateUser::create(db, &payload).await
}

// ➤ get users by id
pub async fn get_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<User>, AppError> {
    User::find_by_id(db, id).await
}

// ➤ get all users
pub async fn get_users(Extension(db): Extension<DB>) -> Result<Json<Vec<User>>, AppError> {
    User::find(db).await.map(Json)
}

// ➤ update user
pub async fn update_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    User::update(db, id, &payload).await.map(Json)
}

// ➤ delete user
pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    let user = User::delete(db, id).await?;
    Ok(user)
}

pub fn router() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
}
