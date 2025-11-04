use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::config::Config;

pub mod test;

pub async fn serve(config: Config) -> Result<(), std::io::Error> {
    let app = api_router().layer(
        ServiceBuilder::new().layer(TraceLayer::new_for_http())
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
        axum::serve(listener, app.into_make_service())
        .await
}

fn api_router() -> Router {
    test::router()
}