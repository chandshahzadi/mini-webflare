use serde::{Serialize, Deserialize};
use sqlx::{FromRow};
use chrono::NaiveDateTime;
use utils::{db::DB, error::AppError};

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

impl Order {

    // create/order
    pub async fn create(
        db: DB,
        payload: CreateOrder,
    ) -> Result<Order, AppError> {
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (user_id, total_price)
            VALUES ($1, $2)
            RETURNING id, user_id, total_price, created_at
            "#,
            payload.user_id,
            payload.total_price     )
        .fetch_one(&db)
        .await?;
        Ok(order)
    }

    // get/order
    pub async fn get(
        db: DB,
        user_id: i32,
    ) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, total_price, created_at
            FROM orders
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&db)
        .await?;

        Ok(orders)
    }

    // update/order
    pub async fn update(
        db: DB,
        id: i32, 
        payload: UpdateOrder,
    ) -> Result<Order, AppError> {
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
        .fetch_one(&db)
        .await?;

        Ok(order)
    }

    // delete/order
    pub async fn delete(
        db: DB,
        id: i32,
    ) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM orders WHERE id = $1",
            id
        )
        .execute(&db)
        .await?;

        Ok(result.rows_affected())
    }

}