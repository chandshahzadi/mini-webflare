use sqlx::PgPool;
use crate::models::cart_repo::{CartRepo, Cart, CreateCart};

// create /cart
pub async fn create_cart(
    pool: &PgPool, 
    payload: CreateCart,
) -> Result<Cart, String> {
    if payload.user_id == 0 {
        return Err("user_id is required".to_string());
    }

    if payload.product_id == 0 {
        return Err("product_id is required".to_string());
    }

    if payload.quantity == 0 {
        return Err("quantity must be greater than 0".to_string());
    }
    
    let result = CartRepo::create_cart(pool, payload).await;

    match result {
        Ok(cart) => Ok(cart),
        Err(_) => Err("Failed to create cart".to_string()),
    }
}

// GET /cart
pub async fn get_cart(
    pool: &PgPool, 
    user_id: i32
) -> Result<Vec<Cart>, String> {
    let result = CartRepo::get_cart(pool, user_id).await;

    match result {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to fetch cart".to_string()),
    }
}

// update /cart
pub async fn update_cart(
    pool: &PgPool,
    id: i32,
    quantity: i32,
) -> Result<Cart, sqlx::Error> {

    if quantity == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    let result = CartRepo::update_cart(pool, id, quantity).await?;

    Ok(result)
}

// delete /cart
pub async fn delete_from_cart(
    pool: &PgPool, 
    id: i32
) -> Result<(), String> {
    let result = CartRepo::delete_from_cart(pool, id).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Delete failed".to_string()),
    }
}