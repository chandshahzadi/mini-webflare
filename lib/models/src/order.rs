use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub user_id: i32,
    pub total_price: f64,
    pub quantity: UpdateOrder,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrder {
    pub total_price: f64,
}

pub struct OrderRepo;

impl OrderRepo {

    // create/order
    pub async fn create_order(
        pool: &PgPool, 
        payload: CreateOrder,
    ) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (user_id, total_price)
            VALUES ($1, $2)
            RETURNING id, user_id, total_price, created_at
            "#,
            payload.user_id,
            payload.total_price     )
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    // get/order
    pub async fn get_orders(
        pool: &PgPool, 
        user_id: i32,
    ) -> Result<Vec<Order>, sqlx::Error> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, total_price, created_at
            FROM orders
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    // update/order
    pub async fn update_order(
        pool: &PgPool, 
        id: i32, 
        payload: UpdateOrder,
    ) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET total_price = COALESCE($1, total_price)
            WHERE id = $2
            RETURNING id, user_id, total_price, created_at
            "#,
            payload.total_price,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    // delete/order
    pub async fn delete_order(
        pool: &PgPool, 
        id: i32,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM orders WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

}