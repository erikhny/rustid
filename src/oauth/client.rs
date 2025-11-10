use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Client {
    pub client_name: String,
    pub client_id: ClientId,
    pub client_secret: Option<ClientSecret>,
    pub client_type: ClientType,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub require_pkce: bool,
}

impl Client {
    pub fn validate_client(
        client_name: String,
        client_id: String,
        client_secret: Option<String>,
        client_type: String,
        redirect_uris: Vec<String>,
        grant_types: Vec<String>,
    ) -> Result<Client, ClientParseError> {
        let client_id = ClientId::from_str(&client_id)?;

        let client_type = ClientType::from_str(&client_type)?;

        if matches!(client_type, ClientType::Confidential) && client_secret.is_none() {
            return Err(ClientParseError::InvalidClientSecret(format!(
                "Client '{}': Confidential clients must have a client_secret",
                client_id.0
            )));
        }

        let client_secret = match client_type {
            ClientType::Public => None,
            ClientType::Confidential => {
                Some(ClientSecret::from_str(&client_secret.unwrap_or_default())?)
            }
        };

        if redirect_uris.is_empty() {
            return Err(ClientParseError::InvalidRedirectUri(format!(
                "Client '{}': Redirect uris is empty",
                client_id.0
            )));
        }

        for uri in &redirect_uris {
            if !uri.starts_with("http://") && !uri.starts_with("https://") {
                return Err(ClientParseError::InvalidRedirectUri(format!(
                    "Client '{}': Invalid URI '{}'",
                    client_id.0, uri
                )));
            }
        }

        let grant_types = grant_types
            .iter()
            .map(|gt| GrantType::from_str(gt))
            .collect::<Result<Vec<_>, _>>()?;

        if grant_types.is_empty() {
            return Err(ClientParseError::InvalidGrantType(format!(
                "Client '{}': At least one grant_type is required",
                client_id.0
            )));
        }

        let require_pkce = false;

        Ok(Client {
            client_name,
            client_type,
            client_id,
            client_secret,
            redirect_uris,
            grant_types,
            require_pkce,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct ClientId(String);
impl ClientId {
    fn from_str(client_id: &str) -> Result<Self, ClientParseError> {
        if client_id.is_empty() {
            return Err(ClientParseError::InvalidClientId(String::from(
                "Client ID is empty",
            )));
        }
        Ok(ClientId(client_id.to_owned()))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct ClientSecret(String);
impl ClientSecret {
    fn from_str(client_secret: &str) -> Result<Self, ClientParseError> {
        if client_secret.is_empty() {
            return Err(ClientParseError::InvalidClientSecret(String::from(
                "Client Secret is empty",
            )));
        }
        Ok(ClientSecret(client_secret.to_owned()))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum ClientType {
    Public,
    Confidential,
}

impl ClientType {
    fn from_str(client_type: &str) -> Result<Self, ClientParseError> {
        match client_type.to_lowercase().as_str() {
            "public" => Ok(ClientType::Public),
            "confidential" => Ok(ClientType::Confidential),
            _ => Err(ClientParseError::InvalidClientType(format!(
                "Client '{}': Invalid client type - should be 'public' or 'confidential'",
                client_type
            ))),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
    ClientCredentials,
}

impl GrantType {
    pub fn from_str(s: &str) -> Result<Self, ClientParseError> {
        match s {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "refresh_token" => Ok(GrantType::RefreshToken),
            "client_credentials" => Ok(GrantType::ClientCredentials),

            "urn:ietf:params:oauth:grant-type:authorization_code" => {
                Ok(GrantType::AuthorizationCode)
            }
            "urn:ietf:params:oauth:grant-type:refresh_token" => Ok(GrantType::RefreshToken),
            "urn:ietf:params:oauth:grant-type:client_credentials" => {
                Ok(GrantType::ClientCredentials)
            }

            _ => Err(ClientParseError::InvalidGrantType(format!(
                "Invalid grant type: {}",
                s
            ))),
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

#[derive(Debug)]
pub enum ClientParseError {
    InvalidClientType(String),
    InvalidGrantType(String),
    InvalidClientSecret(String),
    InvalidClientId(String),
    InvalidRedirectUri(String),
}

impl Display for ClientParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientParseError::InvalidClientType(s) => write!(f, "Invalid client type: {}", s),
            ClientParseError::InvalidGrantType(s) => write!(f, "Invalid grant type: {}", s),
            ClientParseError::InvalidClientSecret(s) => write!(f, "Invalid client secret: {}", s),
            ClientParseError::InvalidClientId(s) => write!(f, "Invalid client id: {}", s),
            ClientParseError::InvalidRedirectUri(s) => write!(f, "Invalid redirect uri: {}", s),
        }
    }
}

impl std::error::Error for ClientParseError {}
pub fn deserialize_clients<'de, D>(deserializer: D) -> Result<HashMap<String, Client>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    struct DeserializedClient {
        name: String,
        #[serde(rename = "type")]
        client_type: String,
        client_secret: Option<String>,
        redirect_uris: Vec<String>,
        grant_types: Vec<String>,
    }

    let raw: HashMap<String, DeserializedClient> = Deserialize::deserialize(deserializer)?;

    raw.into_iter()
        .map(|(client_id, deserialized_client)| {
            let client = Client::validate_client(
                deserialized_client.name,
                client_id.clone(),
                deserialized_client.client_secret,
                deserialized_client.client_type,
                deserialized_client.redirect_uris,
                deserialized_client.grant_types,
            )
            .map_err(serde::de::Error::custom)?;

            Ok((client_id, client))
        })
        .collect()
}
