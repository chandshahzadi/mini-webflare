use axum::{
    extract::{Path,State},
    Json,
};
use mini_webflare::models::cart_repo::{Cart, CreateCart};
 use sqlx::PgPool;
 use actix_web::http::StatusCode;
 use mini_webflare::services::cart_service;
 
// create
pub async fn add_to_cart(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, String> {
    let result = cart_service::create_cart(&pool, payload).await;

    match result {
        Ok(cart) => Ok(Json(cart)),
        Err(e) => Err(e),
    }                                                       
}

// get /cart/:id
pub async fn get_cart(Path(
    user_id): Path<i32>, 
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Cart>>, StatusCode> {
    let result = cart_service::get_cart(&pool, user_id).await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// update/cart/:id
pub async fn update_cart(
    Path(id): Path<i32>, 
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, StatusCode> {
    let result = cart_service::update_cart(&pool, id, payload.quantity).await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            println!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// delete/cart/:id
pub async fn delete_from_cart(
    State(pool): State<PgPool>, 
    Path(id): Path<i32>,
) -> Result<Json<String>, String> {
    mini_webflare::services::cart_service::delete_from_cart(&pool, id).await?;
    Ok(Json(format!("Cart item {} removed", id)))
}