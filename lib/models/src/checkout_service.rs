use serde::{Deserialize, Serialize};
use sqlx::Type;
use utils::{db::DB, error::AppError};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method")]
pub enum PaymentMethod {
    Online,
    CashOnDelivery,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method")]
pub enum ShippingMethod {
    HomeDelivery,
    Pickup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub shipping_method: ShippingMethod,
    pub payment_method: PaymentMethod,
    pub contact_number: String,
}

impl CheckoutRequest {
    pub async fn checkout(self, db: DB, user_id: i32) -> Result<i32, AppError> {
        let mut tx = db.begin().await?;

        println!("user_id: {:?}", tx);

        // get cart items
        let cart_items = sqlx::query!(
            r#"
            SELECT
                ci.product_id,
                ci.quantity,
                p.price
            FROM cart_items ci
            JOIN products p
                ON p.id = ci.product_id
            JOIN carts c
                ON c.id = ci.cart_id
            WHERE c.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&mut *tx)
        .await?;
        println!("cart_items: {:#?}", cart_items);
        if cart_items.is_empty() {
            return Err(AppError::BadRequest("Cart is empty".into()));
        }

        //calculate total price from database
        let total = sqlx::query!(
            r#"
            SELECT SUM(ci.quantity * p.price) AS total_price
            FROM cart_items ci
            JOIN products p
                ON p.id = ci.product_id
            JOIN carts c
                ON c.id = ci.cart_id
            WHERE c.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&mut *tx)
        .await?;

        let total_price = total
            .total_price
            .ok_or_else(|| AppError::BadRequest("Failed to calculate total".into()))?;

        // create order
        let payment_method = match self.payment_method {
            PaymentMethod::Online => "Online",
            PaymentMethod::CashOnDelivery => "CashOnDelivery",
        };

        let shipping_method = match self.shipping_method {
            ShippingMethod::HomeDelivery => "Online",
            ShippingMethod::Pickup => "CashOnDelivery",
        };

        let order = sqlx::query!(
            r#"
                INSERT INTO orders (user_id, total_price, shipping_method, payment_method, contact_number)
                VALUES ($1, $2,  $3,  $4,  $5)
                RETURNING id
            "#,
            user_id,
            total_price,
            shipping_method,
            payment_method,
            self.contact_number
        )
        .fetch_one(&mut *tx)
        .await?;
        println!("order result = {:?}", order);
        let order_id = order.id;

        //create order_items
        for item in cart_items {
            sqlx::query!(
                r#"
                INSERT INTO order_items
                (order_id, product_id, quantity)
                VALUES ($1, $2, $3)
                "#,
                order_id,
                item.product_id,
                item.quantity,
            )
            .execute(&mut *tx)
            .await?;
            println!("item = {:?}", item);
        }

        // clear cart
        let delete = sqlx::query!(
            r#"
            DELETE FROM cart_items
            WHERE cart_id IN (
                SELECT id
                FROM carts
                WHERE user_id = $1
            )
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await;
        println!("delete = {:?}", delete);

        // return order id
        tx.commit().await?;
        Ok(order.id)
    }
}
