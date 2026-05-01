use sqlx::PgPool;
use models::product_repo::{self, CreateProduct, Product};

//create/product
pub async fn create_product(
    pool: &PgPool,
    payload: CreateProduct,
) -> Result<Product, String> {
    if payload.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if payload.price <= 0.0 {
        return Err("Price must be greater than 0".to_string());
    }
    
    //calling db
    let result = product_repo::insert_product(pool, &payload).await;

    match result {
        Ok(product) => Ok(product),
        Err(_) => Err("Failed to create product".to_string()),
    }
}

// get/product
pub async fn get_products(
    pool: &PgPool,
)-> Result<Vec<Product>, String> {
    let result = product_repo::get_products(pool).await;

    match result {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to fetch products".to_string()),
    }
}

//create/product
pub async fn update_product(
    pool: &PgPool,
    id: i32,
    payload: CreateProduct,
) -> Result<(), String> {

    if payload.name.is_empty() {
        return Err("Name required".to_string());
    }

    let result = product_repo::update_product_db(pool, id, &payload).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Update failed".to_string()),
    }
}

//delete/product
pub async fn delete_product(
    pool: &PgPool, 
    id: i32,
)-> Result<(), String> {
    let result = product_repo::delete_product_db(pool, id).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Delete failed".to_string()),
    }
}