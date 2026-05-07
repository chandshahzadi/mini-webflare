use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

// Request body
#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
}
pub struct ProductRepo;

impl ProductRepo {

    // CREATE
    pub async fn insert_product(
 pool: &PgPool, 
         payload: &UpdateProduct,
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
    payload: &UpdateProduct,
    ) -> Result<Product, sqlx::Error> {
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
        .fetch_one(pool)
        .await?;

        Ok(product)
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
}