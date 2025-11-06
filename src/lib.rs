use crate::config::Config;
use crate::http::test;
use axum::{Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod config;
pub mod http;
pub mod oauth;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
}

pub async fn serve(config: Config) -> Result<(), std::io::Error> {
    let app = api_router()
        .layer(Extension(ApiContext {
            config: Arc::new(config),
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app.into_make_service()).await
}

fn api_router() -> Router {
    test::router().merge(oauth::router())
}
