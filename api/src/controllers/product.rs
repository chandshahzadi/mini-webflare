use axum::{
    Extension, Json, Router, extract::Path, routing::{get, post}
};
use utils::{db::DB, error::AppError};
use models::product::{Product, UpdateProduct};

// ➤ create/cart
pub async fn insert_product(
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    let product = Product::create(&db, &payload).await?;
    Ok(Json(product))   
}

// ➤ get/product
pub async fn get_products(
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Product>>, AppError> {
    let product = Product::get(&db).await?;
    Ok(Json(product))
}

pub async fn update_product_db(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, AppError> {
    let product = Product::update(&db, id, &payload).await?;
    Ok(Json(product))
}

// ➤ delete/product
pub async fn delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DB>,
) -> Result<Json<Product>, AppError> {
    Product::delete_product_db(&db, id).await.map(Json)
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/products", get(get_products).post(insert_product))
        .route("/products/:id", get(get_products).put(update_product_db).delete(delete_product_db))
}
