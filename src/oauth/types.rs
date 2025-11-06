#[derive(serde::Serialize, serde::Deserialize)]
struct Client {
    client_name: String,
    client_id: ClientId,
    client_secret: ClientSecret,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClientId(String);
#[derive(serde::Serialize, serde::Deserialize)]
struct ClientSecret(String);
