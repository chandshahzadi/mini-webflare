use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{db::DB, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateCart {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

impl CreateCart {
    // create/cart
    pub async fn create(self, db: DB) -> Result<(), AppError> {
        sqlx::query_as!(
            Cart,
            r#"
            INSERT INTO carts (user_id, product_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, product_id, quantity
            "#,
            self.user_id,
            self.product_id,
            self.quantity
        )
        .fetch_one(&db)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

impl Cart {
    // get/cart
    pub async fn get(db: DB) -> Result<Vec<Cart>, AppError> {
        let carts = sqlx::query_as!(Cart, "SELECT id, user_id, product_id, quantity FROM carts")
            .fetch_all(&db)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(carts)
    }

    // update/cart
    pub async fn update(db: DB, id: i32, quantity: i32) -> Result<Cart, AppError> {
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
}

// delete/cart
pub async fn delete(db: DB, cart_id: i32) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM carts WHERE id = $1", cart_id)
        .execute(&db)
        .await?;
    Ok(())
}
