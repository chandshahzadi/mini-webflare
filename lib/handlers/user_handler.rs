use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
use axum::debug_handler;
use sqlx::PgPool;
use crate::models::user_repo::{User, CreateUser};
use crate::utils::password::hash_password;

// create/user
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, String> {
    let hashed_password = hash_password(&payload.password);

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (first_name,last_name,email,password)
         VALUES ($1, $2, $3, $4)
         RETURNING id, first_name, last_name, email, password"
    )
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.email)
    .bind(&payload.password)
    .bind(&hashed_password)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => {
            println!("User created in DB: {:?}", user); 
            Ok(Json(user))
        },
        Err(e) => {
            eprintln!("DATABASE ERROR: {:?}", e); 
            Err(format!("Failed to create user: {:?}", e))
        }
    }
}

// get/users
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
                eprintln!("DATABASE ERROR: {:?}", pool);

    let users = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, email, password FROM users"
    )
    .fetch_all(&pool)
    .await;

    match users {
        Ok(list) => Ok(Json(list)),
        Err(e) => {
            eprintln!("DATABASE ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// create/user
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, String> {

    let result = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, email, password FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            Err("User not found".to_string())
        }
    }
}
//update/user
#[debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
        State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,   
) -> Result<(), StatusCode> {
    sqlx::query!(
        "
            UPDATE users 
            SET first_name = $1, 
            last_name = $2, 
            email = $3, 
            password = $4
            WHERE id = $5
        ",
        payload.first_name,
        payload.last_name,
        payload.email,
        payload.password,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}

// delete/users 
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<&'static str, String> {

    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
        
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok("User deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting user: {}", e);
            Err("Failed to delete user".to_string())
        }
    }
}