use std::env;
use tokio::net::TcpListener;
use dotenvy::dotenv;

use utils::db::DB;
pub mod routes;
mod authentication;
mod controllers;

#[tokio::main]
async fn main() {
    // dotenv().ok();
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");

    let db = DB::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = routes::router()
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}