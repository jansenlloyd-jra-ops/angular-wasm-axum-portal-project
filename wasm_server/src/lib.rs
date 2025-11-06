use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: String,
    pub id_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionConfig {
    pub provider: String,
    pub scope: String,
    pub provision_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_url: String,
    pub redirect_uri: String,
    pub issuer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenParameters {
    pub code: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenRequest {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub hd: String,
    pub prompt: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Providers {
    pub name: String,
    pub provider: String,
    pub kpc_id: String,
    pub configuration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Users {
    pub name: String,
    pub email: String,
    pub ez_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EncryptionZone {
    pub zone_name: String,
    pub ez_id: String,
    pub kpc_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderRequest {
    pub name: String,
    pub configuration: Value,
}
