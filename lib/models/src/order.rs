use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Type;
use utils::{
    db::{self, DB},
    error::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub total_price: f64,
}

impl CreateOrder {
    // create order for the login user
    pub async fn create(self, db: DB, user_id: i32) -> Result<Order, AppError> {
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (user_id, total_price)
            VALUES ($1, $2)
            RETURNING
                id,
                user_id,
                total_price,
                status as "status: OrderStatus",
                created_at
            "#,
            user_id,
            self.total_price,
        )
        .fetch_one(&db)
        .await?;
        Ok(order)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "order_status", rename_all = "PascalCase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Packed,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub status: OrderStatus,
    pub created_at: Option<NaiveDateTime>,
}

impl Order {
    // find orders belonging to a user
    pub async fn find_by_user_id(db: DB, user_id: i32) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, total_price,
            status as "status: OrderStatus", created_at
            FROM orders
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&db)
        .await?;

        Ok(orders)
    }
    // find all orders admin
    pub async fn find(db: DB) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT
                id,
                user_id,
                total_price,
                status as "status: OrderStatus",
                created_at
            FROM orders
            ORDER BY created_at DESC
            "#
        )
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
    pub total_price: Option<f64>,
    pub status: Option<OrderStatus>,
}

impl UpdateOrder {
    pub async fn update(self, db: DB, id: i32) -> Result<Order, AppError> {
        let order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET total_price = COALESCE($1, total_price),
                status = COALESCE($2, status)
            WHERE id = $3
            RETURNING id, user_id, total_price, status as "status: OrderStatus", created_at
            "#,
            self.total_price,
            self.status as Option<OrderStatus>,
            id
        )
        .fetch_one(&db)
        .await?;
        Ok(order)
    }
}
