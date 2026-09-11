use axum::{
    Extension, Json, Router,
    extract::Path,
    middleware::from_fn,
    routing::{get, put},
};
use models::user::{CreateUser, User};
use utils::{
    db::DB,
    error::AppError,
    middleware::{AuthUser, is_admin},
};

// ➤ get users by id
pub async fn get_user(
    Extension(auth): Extension<AuthUser>,
    Extension(db): Extension<DB>,
) -> Result<Json<User>, AppError> {
    User::find_by_id(db, auth.id).await.map(Json)
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
        .route("/users/me", get(get_user))
        .route("/users", get(get_users).layer(from_fn(is_admin)))
        .route(
            "/users/{id}",
            put(update_user)
                .delete(delete_user)
                .layer(from_fn(is_admin)),
        )
}
