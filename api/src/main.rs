use std::net::SocketAddr;
use tokio::net::TcpListener;

use api::create_app;
use utils::db::connect_db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = connect_db().await;

    let app = create_app(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
