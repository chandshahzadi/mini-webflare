use axum::{
    Extension, Json, Router, extract::Path, routing::{get, post}
};
use utils::{db::DB, error::AppError};
use models::product::{Product, UpdateProduct};

// ➤ create/cart
pub async fn insert_product(
    Extension(db): Extension<DB>,
    Json(payload): Json<Product>,
) -> Result<(), AppError> {
     Product::create(db, &payload).await
}

// ➤ get/product
pub async fn get_products(
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Product>>, AppError> {
    Product::get(db).await.map(Json)
}

pub async fn update_product_db(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
     UpdateProduct::update(db, id, &payload).await.map(Json)
}

// ➤ delete/product
pub async fn delete_product_db(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Product::delete(db, id).await
}

pub fn router() -> Router<DB> {
    Router::new()
        .route("/products", get(get_products).post(insert_product))
        .route("/products/:id", get(get_products).put(update_product_db).delete(delete_product_db))
}
