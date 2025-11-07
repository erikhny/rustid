use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use rustid::config::Config;
use rustid::{create_router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string).await
        .expect("failed to connect to postgres");

    let config = Config::parse();


    let router = create_router(config, pool).await?;
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
