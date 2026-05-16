use serde::{Serialize, Deserialize};
use sqlx::{FromRow};
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

impl Product {

    // create
    pub async fn create(
        db: DB,
        payload: &Product,
    )-> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO products (name, price)
            VALUES ($1, $2)
            RETURNING id, name, price"
        )
        .bind(&payload.name)
        .bind(payload.price)
        .execute(&db)
        .await?;
        Ok(())
    }

    // get
    pub async fn get(
        db: DB,
    )-> Result<Vec<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, price FROM products"
        )
        .fetch_all(&db)
        .await
        .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(product)
    }
    // delete
    pub async fn delete(
        db: DB,
        id: i32
    )-> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM products WHERE id=$1",
            id
        )
        .execute(&db)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
}

impl UpdateProduct {

    // update
    pub async fn update(
    db: DB,
    id: i32,
    payload: &UpdateProduct,
    ) -> Result<Product, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            UPDATE products
            SET name = $1, price = $2
            WHERE id = $3
            RETURNING id, name, price
            "#,
            payload.name,
            payload.price,
            id
        )
        .fetch_one(&db)
        .await?;
        Ok(product)
    }
}

