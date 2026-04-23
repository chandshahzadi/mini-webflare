mod routes;
use axum::Router;
use crate::routes::user_routes::user_routes;
use mini_webflare::migrations::db::connect_db;
use crate::routes::cart_routes::cart_routes;
use crate::routes::order_routes::order_routes;
use crate::routes::product_routes::product_routes;
use crate::routes::auth_routes::auth_route;

#[tokio::main]
async fn main() {
    let pool = connect_db().await;
    let app = Router::new()
        .merge(auth_route())   
        .merge(user_routes()) 
        .merge(cart_routes())
        .merge(order_routes())
        .merge(product_routes())
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}