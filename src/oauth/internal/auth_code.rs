
struct PendingAuthentication {
    client_id: String,
    redirect_uri: Option<String>,
    state: Option<String>,
    scope: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    authorization_code: AuthorizationCode
}

struct AuthorizationCode {
    code: String,
    redirect_uri: Option<String>,
    state: Option<String>,
    client_id: String,
    scope: Option<String>,
    expires_at: chrono::DateTime<chrono::Utc>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

enum GrantType {
    AuthorizationCode,
    RefreshToken,
}