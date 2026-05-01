use axum::{
    Json, extract::{Path,State}, response::IntoResponse
};
use models::cart_repo::{Cart, CreateCart};
use actix_web::http::StatusCode;
 use services::cart_service;
 use utils::db::DB;

 
// create
pub async fn add_to_cart(
    State(db): State<DB>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, String> {
    let result = cart_service::create_cart(&db, payload).await;

    match result {
        Ok(cart) => Ok(Json(cart)),
        Err(e) => Err(e),
    }                                                       
}

pub struct AuthUser {
    pub user_id: i32,
}

// get /cart/:id
pub async fn get_cart(
    State(db): State<DB>,
    auth: AuthUser,
) -> impl IntoResponse {
    let user_id = auth.user_id;

    let result = cart_service::get_cart(&db, user_id).await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// update/cart/:id
pub async fn update_cart(
    Path(id): Path<i32>, 
    State(db): State<DB>,
    Json(payload): Json<CreateCart>,
) -> Result<Json<Cart>, StatusCode> {
    let result = cart_service::update_cart(&db, id, payload.quantity).await;

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
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<Json<String>, String> {
    services::cart_service::delete_from_cart(&db, id).await?;
    Ok(Json(format!("Cart item {} removed", id)))
}