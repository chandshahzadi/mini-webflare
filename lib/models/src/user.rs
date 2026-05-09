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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub quantity: String
}

impl User {
    pub async fn create(
        pool: &PgPool, 
        payload: &CreateUser,
    )-> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING id, first_name, last_name, email, password",
            payload.first_name,
            payload.last_name,
            payload.email,
            payload.password
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get(
        pool: &PgPool,
    )-> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, email, password FROM users"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        payload: &CreateUser,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name=$1,
                last_name=$2,
                email=$3,
                password=$4
            WHERE id=$5
            RETURNING id, first_name, last_name, email, password
            "#,
            payload.first_name,
            payload.last_name,
            payload.email,
            payload.password,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(
        pool: &PgPool, 
        id: i32
    )-> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id=$1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}