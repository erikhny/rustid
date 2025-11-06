use axum::Extension;
use axum::response::Redirect;
use crate::ApiContext;

pub async fn authorize(ctx: Extension<ApiContext>) -> Redirect {
    Redirect::to("/test/authorize")
}