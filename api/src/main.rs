use std::env;
use tokio::net::TcpListener;
use utils::db::DB;
mod authentication;
mod controllers;
pub mod routes;
use axum::Extension;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let db = DB::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app = routes::router().layer(Extension(db));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
