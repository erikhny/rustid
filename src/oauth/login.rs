use crate::ApiContext;
use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthorizeQueryParams {
    response_type: String,
    state: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
}

pub async fn authorize(State(ctx): State<ApiContext>, Query(params): Query<AuthorizeQueryParams>) -> Redirect {
    println!("{:?}", params);

    let client_id = params.client_id.as_str();
    let client = match ctx.client_store.get(client_id) {
        Some(client) => client,
        None => return Redirect::permanent("/"), // TODO: Properly handle error here.
    };

    let redirect_uri = match params.redirect_uri {
        Some(redirect_uri) => redirect_uri,
        None => return Redirect::permanent("/"), // TODO: Properly handle error here.
    };
    if !&client.redirect_uris.contains(&redirect_uri) {
        return Redirect::permanent("/"); // TODO: Properly handle error here.
    }

    if client.require_pkce {
        // TODO: Handle pkce here.
    }

    // Validate scopes?

    // If all validation succeeds:
    // Make decision to redirect to actual login or SSO handling?


    Redirect::to("/test/authorize")
}
