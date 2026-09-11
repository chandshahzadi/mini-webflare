use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateCartItem {
    pub product_id: i32,
    pub quantity: i32,
}

// cart items
impl CreateCartItem {
    pub async fn create_cart_item(self, db: DB, user_id: i32) -> Result<(), AppError> {
        let cart = Cart::get_or_create(&db, user_id).await?;

        sqlx::query!(
            r#"
            INSERT INTO cart_items (cart_id, product_id, quantity)
            VALUES ($1, $2, $3)
            ON CONFLICT (cart_id, product_id) DO UPDATE
                SET quantity = cart_items.quantity + EXCLUDED.quantity
            "#,
            cart.id,
            self.product_id,
            self.quantity
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
}

impl Cart {
    pub async fn get_or_create(db: &DB, user_id: i32) -> Result<Cart, AppError> {
        let cart = sqlx::query_as!(
            Cart,
            r#"
            INSERT INTO carts (user_id)
            VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE
                SET user_id = EXCLUDED.user_id
            RETURNING id, user_id
            "#,
            user_id
        )
        .fetch_one(db)
        .await?;

        Ok(cart)
    }
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
    pub async fn update(
        db: DB,
        id: i32,
        user_id: i32,
        quantity: i32,
    ) -> Result<CartItem, AppError> {
        let item = sqlx::query_as!(
            CartItem,
            r#"
            UPDATE cart_items ci
            SET quantity = $1
            FROM carts c
            WHERE ci.id = $2
              AND c.id = ci.cart_id
              AND c.user_id = $3
            RETURNING ci.id, ci.cart_id, ci.product_id, ci.quantity
            "#,
            quantity,
            id,
            user_id
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(item)
    }
    // DELETE cartItem
    pub async fn delete(db: DB, id: i32, user_id: i32) -> Result<(), AppError> {
        let deleted = sqlx::query!(
            r#"
               DELETE FROM cart_items ci
               USING carts c
               WHERE ci.id = $1
                 AND c.id = ci.cart_id
                 AND c.user_id = $2
               "#,
            id,
            user_id
        )
        .execute(&db)
        .await?;

        if deleted.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
