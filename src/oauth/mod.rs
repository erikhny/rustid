use axum::Router;
use axum::routing::{get, post};
use crate::oauth::login::authorize;
use crate::oauth::token::token;

pub mod types;
mod login;
mod token;

pub fn router() -> Router {
    let oauth_routes = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token));
    Router::new()
        .nest("/oauth", oauth_routes)
}