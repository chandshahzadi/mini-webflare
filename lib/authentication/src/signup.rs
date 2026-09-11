use serde::Deserialize;
use sqlx::query;
use utils::db::DB;
use utils::{encryption::hash_password, error::AppError};

#[derive(Deserialize)]
pub struct Signup {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl Signup {
    pub async fn signup(self, db: DB) -> Result<(), AppError> {
        let hashed = hash_password(&self.password);

        query!(
            r#"
            INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4)
            "#,
            self.first_name,
            self.last_name,
            self.email,
            hashed,
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}
