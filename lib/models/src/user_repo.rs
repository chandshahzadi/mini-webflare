use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)] 
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn insert_user(
    pool: &PgPool, 
    payload: &CreateUser,
)-> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (first_name, last_name, email, password)
         VALUES ($1, $2, $3, $4)
         RETURNING id, first_name, last_name, email, password"
    )
    .bind(payload.first_name.clone())
    .bind(payload.last_name.clone())
    .bind(payload.email.clone())
    .bind(payload.password.clone())
    .fetch_one(pool)
    .await
}

pub async fn get_user(
    pool: &PgPool,
)-> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name, email, password FROM users"
    )
    .fetch_all(pool)
    .await
}

pub async fn update_user(
    pool: &PgPool,
    id: i32,
    payload: &CreateUser,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET first_name=$1, last_name=$2, email=$3, password=$4 WHERE id=$5",
        payload.first_name,
        payload.last_name,
        payload.email,
        payload.password,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_user(
    pool: &PgPool, 
    id: i32
)-> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id=$1", id)
        .execute(pool)
        .await?;
    Ok(())
}