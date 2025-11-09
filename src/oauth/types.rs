use std::collections::HashMap;
use serde::de::Error;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub client_name: String,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub require_pkce: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClientId(String);
impl ClientId {
    fn from_str(client_id: &str) -> Result<Self, String> {
        if client_id.is_empty() {
            return Err(String::from("Client ID is empty"));
        }
        Ok(ClientId(client_id.to_owned()))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClientSecret(String);

#[derive(serde::Serialize, serde::Deserialize)]
enum ClientType {
    Public,
    Confidential
}

impl Client {
    pub fn new(client_name: String, client_id: ClientId) -> Client {}
}

#[derive(serde::Serialize, serde::Deserialize)]
enum GrantType {
    AuthorizationCode,
    RefreshToken,
    ClientCredentials,
}

impl GrantType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "refresh_token" => Ok(GrantType::RefreshToken),
            "client_credentials" => Ok(GrantType::ClientCredentials),

            "urn:ietf:params:oauth:grant-type:authorization_code" => Ok(GrantType::AuthorizationCode),
            "urn:ietf:params:oauth:grant-type:refresh_token" => Ok(GrantType::RefreshToken),
            "urn:ietf:params:oauth:grant-type:client_credentials" => Ok(GrantType::ClientCredentials),

            _ => Err(format!("Invalid grant type: {}", s)),
        }
    }
    pub fn as_urn(&self) -> &'static str {
        match self {
            GrantType::AuthorizationCode => "urn:ietf:params:oauth:grant-type:authorization_code",
            GrantType::RefreshToken => "urn:ietf:params:oauth:grant-type:refresh_token",
            GrantType::ClientCredentials => "urn:ietf:params:oauth:grant-type:client_credentials",
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::RefreshToken => "refresh_token",
            GrantType::ClientCredentials => "client_credentials",
        }
    }
}

fn deserialize_clients<'a, T>(deserializer: T) -> Result<HashMap<String, Client>, T::Error>
where T: serde::de::Deserializer<'a> {

    #[derive(serde::Deserialize)]
    struct DeserializedClient {
        name: String,
        #[serde(rename = "type")]
        client_type: String,
        client_secret: Option<String>,
        redirect_uris: Vec<String>,
        grant_types: Vec<String>,
    }

    let raw: HashMap<String, DeserializedClient> = serde::Deserialize::deserialize(deserializer)?;

    raw.into_iter()
        .map(|(client_id, raw_client)| {
            let client_id_parsed = ClientId::from_str(&client_id).map_err(|e| {
                return Err(T::Error::custom(format!(
                    "Client '{}': failed to parse client_id with err: '{}'", client_id, e
                )))
            })?;

            let client_type = match raw_client.client_type.as_str() {
                "public" => ClientType::Public,
                "confidential" => ClientType::Confidential,
                _ => return Err(T::Error::custom(format!(
                    "Client '{}': Invalid client type '{}'", client_id, raw_client.client_type
                ))),
            };

            if matches!(client_type, ClientType::Confidential) && raw_client.client_secret.is_none() {
                return Err(T::Error::custom(format!("Client '{}': Confidential clients must have a client_secret", client_id)));
            }

            if raw_client.redirect_uris.is_empty() {
                return Err(T::Error::custom(format!("Client '{}': Redirect uris is empty", client_id)));
            }

            for uri in &raw_client.redirect_uris {
                if !uri.starts_with("http://") && !uri.starts_with("https://") {
                    return Err(T::Error::custom(format!("Client '{}': Invalid URI '{}'", client_id, uri)));
                }
            }

            let grant_types: Vec<GrantType>= raw_client.grant_types
                .iter()
                .map(|gt| GrantType::from_str(gt)
                    .map_err(|e| T::Error::custom(format!("Client '{}': {}", client_id, e)))).collect()?;

            if grant_types.is_empty() {
                return Err(T::Error::custom(format!("Client '{}': At least one grant_type is required", client_id)));
            }

            let require_pkce = false;

            Ok((
                &client_id_parsed.0,
                Client {
                    client_name: raw_client.name,
                    client_type,
                    client_id: client_id_parsed,
                    client_secret,
                    redirect_uris: raw_client.redirect_uris,
                    grant_types,
                    require_pkce,
                },
            ))
        })
        .collect()
}