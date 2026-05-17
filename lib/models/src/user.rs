use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl CreateUser {
    pub async fn create(
        db: DB,
        payload: &CreateUser,
    )-> Result<(), AppError> {
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
        .fetch_one(&db)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)] 
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub password: String,
}

impl  User {
    //find user by id
    pub async fn find_by_id(
        db: DB,
        id: i32,
    ) -> Result<Json<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, email, password FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&db)
        .await?;
        Ok(Json(users))
    }

    // find all users
      pub async fn find(
        db: DB,
    ) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, first_name, last_name, email, password FROM users"
        )
        .fetch_all(&db)
        .await?;
        Ok(users)
    }

    pub async fn update(
    db: DB,
    id: i32,
    payload: &CreateUser,
    ) -> Result<User, AppError> {
        let p = sqlx::query_as!(
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
        .fetch_one(&db)
        .await?;
        Ok(p)
    }

    // delete user
  pub async fn delete(
    db: DB,
    id: i32,
) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "DELETE FROM users
         WHERE id = $1
         RETURNING id, first_name, last_name, email, password"
    )
    .bind(id)
    .fetch_optional(&db)
    .await?;
    Ok(user)
}
}


