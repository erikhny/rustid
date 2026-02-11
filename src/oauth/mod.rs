use crate::oauth::login::authorize;
use crate::oauth::token::token;
use axum::Router;
use axum::routing::{get, post};
use crate::ApiContext;

mod login;
mod token;
pub mod client;
mod internal;

pub fn router() -> Router<ApiContext> {
    let oauth_routes = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token));
    Router::new().nest("/oauth", oauth_routes)
}
