use axum::routing::Router;
use tokio::net::TcpListener;
use utils::db::DB;
pub mod routes;
mod authentication;
mod controllers;

#[tokio::main]
async fn main(){

    let app = Router::new();
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}