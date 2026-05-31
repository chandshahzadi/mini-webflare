use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use models::user::{CreateUser, User};
use utils::db::DB;
use utils::encryption::hash_password;
use utils::error::AppError;

// create user
pub async fn create_user(
    State(db): State<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let hashed_password = hash_password(&payload.password);
    
    let user = sqlx::query_as!(
        User,
        "
        INSERT INTO users (first_name,last_name,email,password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, first_name, last_name, email, password
        ",
        payload.first_name,
        payload.last_name,
        payload.email,
        hashed_password
    )
    .fetch_one(&db)
    .await?;
    Ok(Json(user))
    
}

// get users
pub async fn get_users(State(db): State<DB>) -> Result<Json<Vec<User>>, StatusCode> {
    let users =
        sqlx::query_as!(
            User,
            "SELECT id, first_name, last_name, email, password FROM users"
        )
        .fetch_all(&db)
        .await;

    match users {
        Ok(list) => Ok(Json(list)),
        Err(e) => {
            eprintln!("DATABASE ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// create user
pub async fn get_user(
    State(db): State<DB>, 
    Path(id): Path<i32>
) -> Result<Json<User>, String> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, first_name, last_name, email, password 
        FROM users 
        WHERE id = $1",
        id
    )
    .fetch_one(&db)
    .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            Err("User not found".to_string())
        }
    }
}

//update user
#[debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    State(db): State<DB>,
    Json(payload): Json<CreateUser>,
) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        UPDATE users 
        SET first_name = $1, 
            last_name = $2, 
            email = $3, 
            password = $4
        WHERE id = $5
        "#,
        payload.first_name,
        payload.last_name,
        payload.email,
        payload.password,
        id
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

// delete users
pub async fn delete_user(
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<&'static str, String> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&db)
        .await;

    match result {
        Ok(_) => Ok("User deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting user: {}", e);
            Err("Failed to delete user".to_string())
        }
    }
}
