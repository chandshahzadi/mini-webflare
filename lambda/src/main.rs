use api::create_app;
use axum_aws_lambda::LambdaLayer;
use lambda_http::{Error, run};
use tower::Layer;
use utils::db::connect_db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Lambda starting");

    let db = connect_db().await;

    println!("Database connected");

    let app = create_app(db);

    println!("Router created");

    let service = LambdaLayer::default().layer(app);

    println!("Starting lambda runtime");

    run(service).await?;

    Ok(())
}
