use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{
    db::{self, DB},
    error::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub user_id: i32,
    pub total_price: f64,
}

impl CreateOrder {
    // create order
    pub async fn create(db: DB, payload: CreateOrder) -> Result<(), AppError> {
        sqlx::query_as!(
            Order,
            r#"
        INSERT INTO orders (user_id, total_price)
        VALUES ($1, $2)
        RETURNING id, user_id, total_price, created_at
        "#,
            payload.user_id,
            payload.total_price
        )
        .fetch_one(&db)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub created_at: Option<NaiveDateTime>,
}

impl Order {
    // find orders by id
    pub async fn get(db: DB, user_id: i32) -> Result<Vec<Order>, AppError> {
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
    // find all orders
    pub async fn find(db: DB) -> Result<Vec<Order>, AppError> {
        let orders =
            sqlx::query_as::<_, Order>("SELECT id, user_id, total_price, created_at FROM orders")
                .fetch_all(&db)
                .await?;
        Ok(orders)
    }

    // delete order
    pub async fn delete(db: DB, id: i32) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM orders WHERE id = $1", id)
            .execute(&db)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrder {
    pub total_price: f64,
}

impl UpdateOrder {
    // update/order
    pub async fn update(self, db: DB, id: i32) -> Result<Order, AppError> {
        let order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET total_price = $1
            WHERE id = $2
            RETURNING id, user_id, total_price, created_at
            "#,
            self.total_price,
            id
        )
        .fetch_one(&db)
        .await?;
        Ok(order)
    }
}
