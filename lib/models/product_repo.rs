use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64
}

// Request body
#[derive(Debug, Deserialize, FromRow)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
    pub(crate) user_id: (),
}

// CREATE
pub async fn insert_product(
    pool: &PgPool, 
    payload: &CreateProduct,
)-> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, price)
         VALUES ($1, $2)
         RETURNING id, name, price"
    )
    .bind(&payload.name)
    .bind(payload.price)
    .fetch_one(pool)
    .await
}

// GET ALL
pub async fn get_products(
    pool: &PgPool,
)-> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        "SELECT id, name, price FROM products"
    )
    .fetch_all(pool)
    .await
}

// UPDATE
pub async fn update_product_db(
    pool: &PgPool,
    id: i32,
    payload: &CreateProduct,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE products SET name=$1, price=$2 WHERE id=$3",
        payload.name,
        payload.price,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

// DELETE
pub async fn delete_product_db(
    pool: &PgPool, 
    id: i32
)-> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM products WHERE id=$1", id)
        .execute(pool)
        .await?;
    Ok(())
}