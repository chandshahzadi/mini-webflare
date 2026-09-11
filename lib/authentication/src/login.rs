use serde::Deserialize;
use serde::Serialize;
use sqlx::query;
use std::str::FromStr;
use utils::{db::DB, encryption::verify_password, enums::Role, error::AppError, jwt::create_token};

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(self, db: DB) -> Result<LoginResponse, AppError> {
        let res = query!(
            "SELECT id, password, role FROM users WHERE email = $1",
            self.email
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Unauthorized)?;
        if !verify_password(&self.password, &res.password) {
            return Err(AppError::Unauthorized);
        }

        let role = Role::from_str(&res.role)?;
        let token = create_token(res.id, role).map_err(|e| AppError::DbError(e.to_string()))?;
        // let token =
        //     create_token(res.id, Role::User).map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(LoginResponse { token })
    }
}
