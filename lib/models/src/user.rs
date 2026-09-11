use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    //find user by
    pub async fn find_by_id(db: DB, id: i32) -> Result<User, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, email FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&db)
        .await?;
        Ok(users)
    }

    // find all users
    pub async fn find(db: DB) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>("SELECT id, first_name, last_name, email FROM users")
            .fetch_all(&db)
            .await?;
        Ok(users)
    }

    pub async fn update(db: DB, id: i32, payload: &CreateUser) -> Result<User, AppError> {
        let update = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name=$1,
                last_name=$2,
                email=$3
            WHERE id=$4
            RETURNING id, first_name, last_name, email
            "#,
            payload.first_name,
            payload.last_name,
            payload.email,
            id
        )
        .fetch_one(&db)
        .await?;
        Ok(update)
    }

    // delete user
    pub async fn delete(db: DB, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            id
        )
        .execute(&db)
        .await?;
        Ok(())
    }
}
