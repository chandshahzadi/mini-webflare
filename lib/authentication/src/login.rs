
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use utils::jwt::Claims;

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

impl LoginInput {
    pub async fn login(
        Json(_payload): Json<LoginInput>,
    ) -> Json<String> {
        let claims = Claims {
            user_id: 12,
            role: "user".to_string(),
            exp: (Utc::now().timestamp() + 3600) as usize, 
        };

        let secret = "your_secret_key";
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

        Json(token)
    }
}