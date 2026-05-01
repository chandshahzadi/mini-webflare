pub mod controllers;
mod routes;
use axum::Router;
pub mod routes;

#[tokio::main]
async fn main() {
    let pool = connect_db().await;
    let app = routes::router(connection);
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())






    // let app = Router::new()
    //     .merge(auth_route())   
    //     .merge(user_routes()) 
    //     .merge(cart_routes())
    //     .merge(order_routes())
    //     .merge(product_routes())
    //     .with_state(pool);

    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}