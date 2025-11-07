use crate::ApiContext;
use axum::extract::State;
use axum::response::Redirect;

pub async fn authorize(State(ctx): State<ApiContext>) -> Redirect {
    Redirect::to("/test/authorize")
}
