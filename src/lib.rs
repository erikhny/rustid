use crate::config::Config;
use crate::http::test;
use axum::{Router};
use std::sync::Arc;
use sqlx::PgPool;
use tower::{ServiceBuilder};
use tower_http::trace::TraceLayer;

pub mod config;
pub mod http;
pub mod oauth;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn create_router(config: Config, db_pool: PgPool) -> Result<Router, std::io::Error> {
    let app_context = ApiContext {
        config: Arc::new(config),
        db: db_pool,
    };
    let app = api_router()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_context);
    Ok(app)
}

fn api_router() -> Router<ApiContext> {
    test::router().merge(oauth::router())
}
