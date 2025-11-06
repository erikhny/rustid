use crate::ApiContext;
use axum::Extension;
use axum::response::Redirect;

pub async fn authorize(ctx: Extension<ApiContext>) -> Redirect {
    Redirect::to("/test/authorize")
}
