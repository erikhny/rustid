use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use rustid::config::Config;

#[tokio::test]
async fn test_authorize() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/oauth/authorize", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_redirection());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    let port = listener.local_addr().expect("could not get local address").port();

    let _ = tokio::spawn(async move {
        let config = Config::parse();
        let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string).await
            .expect("failed to connect to postgres");
        let router = rustid::create_router(config, pool).await.expect("failed to create router");
        let listener = TcpListener::from_std(listener).expect("failed to set listener");

        axum::serve(listener, router).await.expect("failed to serve app");
    });

    format!("http://127.0.0.1:{}", port)
}