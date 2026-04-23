use axum::{Json, extract::{Path, State}, http::StatusCode};
use sqlx::PgPool;
use crate::models::product_repo::{Product, CreateProduct};
use crate::services::product_service;

// create/product
pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProduct>,
) -> Result<Json<Product>, String> {

    let result = product_service::create_product(&pool, payload).await;

    match result {
        Ok(product) => Ok(Json(product)),
        Err(e) => Err(e),
    }                                                       
}

// get/product
pub async fn get_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, StatusCode> {

    let result = product_service::get_products(&pool).await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// update/product
pub async fn update_product(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProduct>,
) -> Result<(), StatusCode> {

    let result = product_service::update_product(&pool, id, payload).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// delete/product
pub async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<&'static str, String> {

    let result = product_service::delete_product(&pool, id).await;

    match result {
        Ok(_) => Ok("Product deleted successfully"),
        Err(e) => Err(e),
    }
}