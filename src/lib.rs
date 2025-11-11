use std::collections::HashMap;
use std::sync::Arc;
use crate::config::{Settings};
use crate::http::test;
use axum::{Router};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower::{ServiceBuilder};
use tower_http::trace::TraceLayer;
use crate::oauth::client::Client;

pub mod config;
pub mod http;
pub mod oauth;

#[derive(Clone)]
pub struct ApiContext {
    db: PgPool,
    client_store: Arc<HashMap<String, Client>>
}

pub async fn create_router(configuration: Settings) -> Result<Router, std::io::Error> {
    let connection_string = configuration.database.connection_string();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string).await
        .expect("failed to connect to postgres");
    let app_context = ApiContext {
        db: pool,
        client_store: Arc::new(configuration.clients),
    };
    let app = api_router()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_context);
    Ok(app)
}

fn api_router() -> Router<ApiContext> {
    test::router().merge(oauth::router())
}
