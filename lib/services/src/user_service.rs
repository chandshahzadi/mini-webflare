use sqlx::PgPool;
use models::user_repo::{User, CreateUser};
use models::user_repo;

// create/user
pub async fn create_user(
    pool: &PgPool, 
    payload: CreateUser,
) -> Result<User, String> {
   if payload.first_name.is_empty() {
        return Err("last_name is required".to_string());
    }

    if payload.last_name.is_empty() {
        return Err("last_name is required".to_string());
    }

    if payload.email.is_empty() {
        return Err("last_name is required".to_string());
    }
    
    if payload.password.is_empty() {
        return Err("last_name is required".to_string());
    }
    
    //calling db
    let result = user_repo::insert_user(pool, &payload).await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed to create user".to_string()),
    }
}

// get/user
pub async fn get_users(
    pool: &PgPool)
-> Result<Vec<User>, String> {
    let result = user_repo::get_user(pool).await;

    match result {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to fetch users".to_string()),
    }
}

// update/user
pub async fn update_user(
    pool: &PgPool, 
    id: i32, 
    payload: CreateUser,
) -> Result<(), String> {
    if payload.first_name.is_empty()
        || payload.last_name.is_empty()
        || payload.email.is_empty()
        || payload.password.is_empty()
    {
        return Err("All fields are required".to_string());
    }
    let result = user_repo::update_user(pool, id, &payload).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Update failed".to_string()),
    }
}

// delete/user
pub async fn delete_user(
    pool: &PgPool, 
    id: i32,
)-> Result<(), String> {
    let result = user_repo::delete_user(pool, id).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Delete failed".to_string()),
    }
}