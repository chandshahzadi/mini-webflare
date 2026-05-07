
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, Result};
use sqlx::FromRow;

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

pub struct CartRepo;

impl CartRepo {

    // create/cart
    pub async fn insert_to_cart(
        pool: &PgPool, 
        new_cart: CreateCart,
    ) -> Result<Cart> {
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
        .fetch_one(pool)
        .await?;
                println!("add cart: {:?}", cart);

        Ok(cart)
    }

    // get/cart
    pub async fn get_cart(
        pool: &PgPool, 
        user_id: i32,
    ) -> Result<Vec<Cart>> {
        let carts = sqlx::query_as!(
            Cart,
            r#"
            SELECT id, user_id, product_id, quantity
            FROM carts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        println!("get cart: {:?}", carts);
        Ok(carts)
    }

    // update/cart
    pub async fn update_cart(
        pool: &PgPool, 
        id: i32, 
        quantity: i32,
    ) -> Result<Cart, sqlx::Error> {
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
        .fetch_one(pool)
        .await?;
        println!("updated cart: {:?}", cart);
        Ok(cart)
    }

    // delete/cart
    pub async fn delete_from_cart(
        pool: &PgPool, 
        cart_id: i32,
    ) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM carts WHERE id = $1",
            cart_id
        )
        .execute(pool)
        .await?;
            println!("delete cart: {:?}", result);

        Ok(result.rows_affected())
    }
}