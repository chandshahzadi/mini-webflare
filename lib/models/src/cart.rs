use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateCart {
    pub user_id: i32,
}

impl CreateCart {
    pub async fn create_cart(self, db: DB) -> Result<Cart, AppError> {
        println!("Creating cart for user_id = {:?}", self.user_id);

        let cart = sqlx::query_as!(
            Cart,
            r#"
            INSERT INTO carts (user_id)
            VALUES ($1)
            RETURNING id, user_id
            "#,
            self.user_id
        )
        .fetch_one(&db)
        .await?;

        Ok(cart)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

// cart items
impl CreateCartItem {
    pub async fn create_cart_item(self, db: DB) -> Result<(), AppError> {
        // println!(
        //     "Creating cart item: cart_id = {:?}, product_id = {:?}, quantity = {:?}",
        //     self.cart_id, self.product_id, self.quantity
        // );

        sqlx::query_as!(
            CartItem,
            r#"
            INSERT INTO cart_items (cart_id, product_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING id, cart_id, product_id, quantity
            "#,
            self.cart_id,
            self.product_id,
            self.quantity
        )
        .fetch_one(&db)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct CartResult {
    pub id: i32,
    pub cart_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub product_name: String,
    pub product_price: f64,
    pub cart_item_id: i32,
}

impl CartResult {
    pub async fn find_by_cart_id(db: DB, cart_id: i32) -> Result<Vec<CartResult>, AppError> {
        let cart = sqlx::query_as!(
            CartResult,
            r#"
            SELECT
                ci.id,
                c.id as cart_id,
                c.user_id,
                ci.product_id,
                ci.quantity,
                ci.id as cart_item_id,
                p.name as product_name,
                p.price as product_price
            FROM carts c
            JOIN cart_items ci
                ON ci.cart_id = c.id
            LEFT JOIN products p
                ON p.id = ci.product_id
            WHERE c.id = $1
            "#,
            cart_id
        )
        .fetch_all(&db)
        .await?;
        Ok(cart)
    }

    // fin by id
    pub async fn find_by_user_id(db: DB, user_id: i32) -> Result<Vec<CartResult>, AppError> {
        let cart = sqlx::query_as!(
            CartResult,
            r#"
            SELECT
                c.id as cart_id,
                c.user_id,
                ci.product_id,
                ci.id,
                ci.quantity,
                ci.id as cart_item_id,
                p.name as product_name,
                p.price as product_price
            FROM carts c
            JOIN cart_items ci
                ON ci.cart_id = c.id
            LEFT JOIN products p
                ON p.id = ci.product_id
            WHERE c.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&db)
        .await?;
        Ok(cart)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

impl CartItem {
    // update query
    pub async fn update(db: DB, id: i32, quantity: i32) -> Result<CartItem, AppError> {
        let item = sqlx::query_as!(
            CartItem,
            r#"
            UPDATE cart_items
            SET quantity = $1
            WHERE id = $2
            RETURNING id, cart_id, product_id, quantity
            "#,
            quantity,
            id
        )
        .fetch_one(&db)
        .await?;

        Ok(item)
    }
    // DELETE cartItem
    pub async fn delete(db: DB, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
               DELETE FROM cart_items
               WHERE id = $1
               "#,
            id
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}
