use models::order_repo::OrderRepo;
use sqlx::PgPool;
use models::order_repo::{Order, CreateOrder, UpdateOrder};

// create order
pub async fn create_order(
    pool: &PgPool, 
    payload: CreateOrder,
) -> Result<Order, String> {
    if payload.user_id == 0 {
        return Err("user_id must be greater than 0".to_string());
    }

    if payload.total_price == 0.0 {
        return Err("total_price must be greater than 0".to_string());
    }

    let order = OrderRepo::create_order(pool, payload).await;

    match order {
        Ok(order) => Ok(order),
        Err(_) => Err("Failed to create order".to_string()),
    }
}

// get/order
pub async fn get_orders(
    pool: &PgPool, 
    user_id: i32,
) -> Result<Vec<Order>, String> {
    let orders = OrderRepo::get_orders(pool, user_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(orders)
}

// update/order
pub async fn update_order(
    pool: &PgPool, 
    id: i32, 
    payload: UpdateOrder,
) -> Result<Order, String> {
    if payload.total_price == 0.0 {
        return Err("total_price must be greater than 0".to_string());
    }

    let order = OrderRepo::update_order(pool, id, payload)
        .await
        .map_err(|e| e.to_string())?;

    Ok(order)
}

// delete/order
pub async fn delete_order(
    pool: &PgPool,
    id: i32,
) -> Result<(), String> {
     let rows = OrderRepo::delete_order(pool, id)
        .await
        .map_err(|e| e.to_string())?;
    
    if rows == 0 {
        return Err("Order not found".to_string());
    }
    Ok(())
}