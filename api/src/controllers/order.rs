use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use models::order_repo::{OrderRepo, CreateOrder, UpdateOrder, Order};
use utils::db::DB;

// CREATE
pub async fn create_order(
    State(db): State<DB>,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, StatusCode> {

    match OrderRepo::create_order(&db, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// GET
pub async fn get_orders(
    State(db): State<DB>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Order>>, StatusCode> {

    match OrderRepo::get_orders(&db, user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// UPDATE
pub async fn update_order(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, StatusCode> {

    match OrderRepo::update_order(&db, id, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// DELETE
pub async fn delete_order(
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<Json<String>, Json<String>> {

    match services::order_service::delete_order(&db, id).await {
    Ok(_) => Ok(Json("order deleted successfully".to_string())),        
    Err(e) => Err(Json(e)),
    }
}