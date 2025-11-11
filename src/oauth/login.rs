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
    let client = ctx.client_store.get(client_id);
    if let Some(client) = client {
        let cid = &client.client_id;
       return Redirect::to(format!("/test/{cid}").as_str());
    }
    Redirect::to("/test/authorize")
}
