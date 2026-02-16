use crate::oauth::client::GrantType;

// TODO: Implement PKCE support later
struct PendingAuthentication {
    id: String,
    client_id: String,
    redirect_uri: Option<String>,
    state: Option<String>,
    scope: Option<String>,
    grant_type: GrantType,
    code_challenge: Option<String>,
}

struct AuthorizationCode {
    code: String,
    client_id: String,
    redirect_uri: String,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}