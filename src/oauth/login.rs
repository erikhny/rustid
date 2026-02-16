use crate::ApiContext;
use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;
use crate::oauth::client::{Client, GrantType};

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

    // Won't trigger for now since require_pkce will always be false
    if client.require_pkce {
        // TODO: Handle pkce here.
    }

    let grant_type = match GrantType::from_str(params.response_type.as_str()) {
        Ok(grant_type) => grant_type,
        Err(e) => return Redirect::permanent("/"),
    };
    // Validate scopes?

    let valid_scopes = match &client.scope {
        Some(client_scopes) => {
            let requested_scopes: Vec<&str> = params.scope.as_ref().map_or(vec![], |s| s.split_whitespace().collect());
            for scope in &requested_scopes {
                if !client_scopes.contains_key(*scope) {
                    return Redirect::permanent("/"); // TODO: Properly handle error here.
                }
            }
            requested_scopes
        }
        None => vec![],
    };

    // This should redirect to the actual login page, which will then redirect back to the client with the auth code after successful login.
    Redirect::to("/test/authorize")
}
