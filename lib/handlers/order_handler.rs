use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use sqlx::PgPool;
use crate::models::order_repo::{OrderRepo, CreateOrder, UpdateOrder, Order};
use crate::services::order_service;

// CREATE
pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, StatusCode> {

    match OrderRepo::create_order(&pool, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// GET
pub async fn get_orders(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Order>>, StatusCode> {

    match OrderRepo::get_orders(&pool, user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// UPDATE
pub async fn update_order(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, StatusCode> {

    match OrderRepo::update_order(&pool, id, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// DELETE
pub async fn delete_order(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<String>, Json<String>> {

    match crate::services::order_service::delete_order(&pool, id).await {
    Ok(_) => Ok(Json("order deleted successfully".to_string())),        
    Err(e) => Err(Json(e)),
    }
}