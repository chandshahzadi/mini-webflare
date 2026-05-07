use axum::{Json, extract::State};
use serde::Deserialize;
use utils::encryption::{hash_password};
use utils::db::DB;
use utils::jwt::create_token;

#[derive(Deserialize)]
pub struct SignupInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn signup(
    State(db): State<DB>, 
    Json(payload): Json<SignupInput>
) -> Json<String> {
    let hashed = hash_password(&payload.password);

    let user = sqlx::query!(
        "INSERT INTO users (first_name, last_name, email, password, role) VALUES ($1, $2, $3, $4, 'user') RETURNING id",
        payload.first_name,
        payload.last_name,
        payload.email,
        hashed
    )
    .fetch_one(&db)
    .await
    .expect("Failed to insert user");

    let token = create_token(user.id, "user".to_string());
    Json(token)
}