use crate::ApiContext;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, debug_handler};
use serde::{Deserialize, Serialize};
use axum::extract::State;
use thiserror::Error;

#[derive(Deserialize, Debug, Serialize)]
pub struct TokenResponse {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
}

// Define TokenError enum
#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Invalid request")]
    Invalid,
    #[error("Token invalid")]
    TokenInvalid,
    #[error("Internal server error")]
    Internal,
}

// Way to convert each Enum case into an actual json response
impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TokenError::Invalid => (StatusCode::BAD_REQUEST, "Invalid request"),
            TokenError::TokenInvalid => (StatusCode::UNAUTHORIZED, "Token invalid"),
            TokenError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (
            status,
            Json(TokenErrorResponse {
                error: error_message.to_string(),
            }),
        )
            .into_response()
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TokenErrorResponse {
    error: String,
}

// Return enum from here.
#[debug_handler]
pub async fn token(State(ctx): State<ApiContext>) -> Result<Json<TokenResponse>, TokenError> {
    Ok(Json(TokenResponse {
        access_token: String::from("123"),
        refresh_token: None,
    }))
}
