use crate::oauth::login::authorize;
use crate::oauth::token::token;
use axum::Router;
use axum::routing::{get, post};

mod login;
mod token;
pub mod types;

pub fn router() -> Router {
    let oauth_routes = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token));
    Router::new().nest("/oauth", oauth_routes)
}
