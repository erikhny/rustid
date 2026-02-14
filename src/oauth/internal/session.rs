pub struct Session {
    pub user_id: i32,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}