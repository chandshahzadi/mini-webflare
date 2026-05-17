use axum::{ 
     extract::{Path, State}, Json, Router, middleware, routing::{get, post, put}
};
use utils::{db::DB, error::AppError, middleware::auth_middleware};
use models::user::{CreateUser, User};

// ➤ create/cart
pub async fn create_user(
    State(db): State<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<(), AppError> {
     CreateUser::create(db, &payload).await
}

// ➤ get users by id
pub async fn get_user(
    Path(id): Path<i32>,
    State(db): State<DB>,
) -> Result<Json<User>, AppError> {
     User::find_by_id(db, id).await
}

// ➤ get all users
pub async fn get_users(
    State(db): State<DB>,
) -> Result<Json<Vec<User>>, AppError> {
     User::find(db).await.map(Json)
}

// ➤ update/product 
pub async fn update_user(
    Path(id): Path<i32>,
    State(db): State<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
     User::update(db, id, &payload).await.map(Json)
}

// ➤ delete/product
pub async fn delete_user(
    Path(id): Path<i32>,
    State(db): State<DB>,
) -> Result<Json<Option<User>>, AppError> {
    let user = User::delete(db, id).await?;
    Ok(Json(user))
}

pub fn router() -> Router<DB> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .layer(middleware::from_fn(auth_middleware))
}