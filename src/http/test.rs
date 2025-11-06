use axum::Router;
use axum::extract::Path;
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/test/{test_value}", get(test_route))
}

async fn test_route(Path(test_value): Path<String>) -> String {
    format!("Hello, {}!", test_value)
}
