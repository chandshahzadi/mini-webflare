
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCart {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]   
pub struct UpdateCart {
    pub quantity: i32,
}

impl Cart {
    
    // create/cart
    pub async fn create(
        db: DB, 
        new_cart: CreateCart,
    ) -> Result<Cart, AppError> {
        let cart = sqlx::query_as!(
            Cart,
            r#"
            INSERT INTO carts (user_id, product_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, product_id, quantity
            "#,
            new_cart.user_id,
            new_cart.product_id,
            new_cart.quantity
        )
        .fetch_one(&db)
        .await?;
        println!("add cart: {:?}", cart);
        Ok(cart)
    }

    // get/cart
    pub async fn get(
        db: DB, 
        user_id: i32,
    ) -> Result<Vec<Cart>, AppError> {
        let carts = sqlx::query_as!(
            Cart,
            r#"
            SELECT id, user_id, product_id, quantity
            FROM carts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&db)
        .await?;
        println!("get cart: {:?}", carts);
        Ok(carts)
    }

    // update/cart
    pub async fn update(
        db: DB, 
        id: i32, 
        quantity: i32,
    ) -> Result<Cart, AppError> {
        let cart = sqlx::query_as!(
            Cart,
            r#"
            UPDATE carts
            SET quantity = $1
            WHERE id = $2
            RETURNING id, user_id, product_id, quantity
            "#,
            quantity,
            id
        )
        .fetch_one(&db)
        .await?;
        println!("updated cart: {:?}", cart);
        Ok(cart)
    }

    // delete/cart
    pub async fn delete(
        db: DB, 
        cart_id: i32,
    ) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM carts WHERE id = $1",
            cart_id
        )
        .execute(&db)
        .await?;
            println!("delete cart: {:?}", result);

        Ok(result.rows_affected())
    }
}