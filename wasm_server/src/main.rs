use wasm_server::{AddUserRequest, CreateEZRequest, EncryptionZone, GoogleTokenRequest, ProviderRequest, Providers, ProvisionConfig, Users};
use axum::{
    Json, Router, body::to_bytes, extract::{Path, Query, Request}, response::{Html, IntoResponse}, routing::{get, get_service, post}
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use sha3::{Digest, Sha3_256};
use lazy_static::lazy_static; 
use tokio::sync::Mutex;
use std::{net::SocketAddr, vec};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use uuid::Uuid;
use reqwest::Client;


lazy_static! {
    pub static ref AUTH_TOKEN_VECTOR: Mutex<Vec<String>> = Mutex::new(Vec::new());
    
    pub static ref PROVISION_VECTOR: Mutex<ProvisionConfig> = Mutex::new(ProvisionConfig {
        provider: "".to_string(),
        scope: "".to_string(),
        provision_type: "".to_string(),
        client_id: "".to_string(),
        client_secret: "".to_string(),
        discovery_url: "".to_string(),
        redirect_uri: "".to_string(),
        issuer: "".to_string(),
    });

    pub static ref PROVIDER_VECTOR: Mutex<Vec<Providers>> = Mutex::new(vec![
        Providers {
            name: "key_config_1".to_string(),
            provider: "google".to_string(),
            kpc_id: "c3a59b16-1f6e-4c12-9f0d-5b74b1f146ae".to_string(),
            configuration: r#"{
                "type": "service_account",
                "project_id": "project_id",
                "private_key_id": "private_key_id",
                "private_key": "private_key",
                "client_email": "client@yourdomain.iam.gserviceaccount.com",
                "client_id": "client_id",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token",
                "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
                "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/client%40yourdomain.iam.gserviceaccount.com",
                "universe_domain": "googleapis.com"
            }"#.to_string(),
        },
        Providers {
            name: "key_config_2".to_string(),
            provider: "aws".to_string(),
            kpc_id: "7d0a9323-92ff-4de8-9a67-38e9c6e10b7e".to_string(),
            configuration: r#"{
                "provider_name": "provider_name",
                "region": "region",
                "access_key_id": "access_key_id_default",
                "secret_access_key": "secret_access_key_default"
            }"#.to_string(),
        },
        Providers {
            name: "key_config_3".to_string(),
            provider: "aws".to_string(),
            kpc_id: "2fdd89ac-3a47-4b5f-8a11-9cfb2d94a052".to_string(),
            configuration: r#"{
                "provider_name": "provider_name",
                "region": "region",
                "access_key_id": "access_key_id_default",
                "secret_access_key": "secret_access_key_default"
            }"#.to_string(),
        },
        Providers {
            name: "key_config_4".to_string(),
            provider: "azure".to_string(),
            kpc_id: "b4c76267-ef1b-4e63-bb03-1af5a6c4a1d4".to_string(),
            configuration: r#"{
                "vault_url": "vault_default_url",
                "tenant_id": "tenant1_id",
                "client_id": "client_id_default",
                "client_secret": "client_secret_default"
            }"#.to_string(),
        }
    ]);

    pub static ref USER_VECTOR: Mutex<Vec<Users>> = Mutex::new(vec![  
        Users { name: "Alice Johnson".to_string(), email: "alice.johnson@example.com".to_string(), ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string()},
        Users { name: "Brian Kim".to_string(), email: "brian.kim@example.com".to_string(), ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string()},
        Users { name: "Carla Mendoza".to_string(), email: "carla.mendoza@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "David Smith".to_string(), email: "david.smith@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "Carol White".to_string(), email: "carol@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "David Brown".to_string(), email: "david@example.com".to_string(), ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string()},
        Users { name: "Eve Green".to_string(), email: "eve@example.com".to_string(), ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string()},
        Users { name: "Frank Blue".to_string(), email: "frank@example.com".to_string(), ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string()}
    ]);
    pub static ref EZ_VECTOR: Mutex<Vec<EncryptionZone>> = Mutex::new(vec![  
    EncryptionZone {
        zone_name: "test_department1".to_string(),
        ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string(),
        kpc_id: "c3a59b16-1f6e-4c12-9f0d-5b74b1f146ae".to_string(),
    },
    EncryptionZone {
        zone_name: "test_department2".to_string(),
        ez_id: "0c3b6e42-2c41-4b7d-9a9b-4a3c583f97c1".to_string(),
        kpc_id: "7d0a9323-92ff-4de8-9a67-38e9c6e10b7e".to_string(),
    },
    EncryptionZone {
        zone_name: "test_department3".to_string(),
        ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string(),
        kpc_id: "2fdd89ac-3a47-4b5f-8a11-9cfb2d94a052".to_string(),
    },
    EncryptionZone {
        zone_name: "test_department4".to_string(),
        ez_id: "9f87c9e4-bc71-4f3d-9b1b-023e3a745ee6".to_string(),
        kpc_id: "b4c76267-ef1b-4e63-bb03-1af5a6c4a1d4".to_string(),
    },
    ]);
}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WrapResp {
    pub wrapped_key: String,
    pub msg_id: Option<String>,
}

pub async fn make_opaque(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    OsRng.fill_bytes(&mut buf);
    URL_SAFE_NO_PAD.encode(&buf)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // or restrict below
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        // .fallback_service(ServeDir::new("assets/admin_portal"));
        .nest_service(
            "/v1/portal",
            get_service(ServeDir::new("assets/admin_portal")),
        )
        .route("/v1/portal/kms-configuration/{config}", post(configure_kpc))
        .route("/v1/portal/provision-configuration", post(configure_provision))
        .route("/v1/portal/kpc", post(return_kpcs))
        .route("/v1/portal/users", post(return_users))
        .route("/v1/portal/ez", post(return_ez))
        .route("/v1/portal/authorization", post(check_authorization))
        .route("/v1/portal/callback", get(token_request))
        .route("/v1/portal/callback/authorize", get(print_raw))
        .route("/v1/portal/add-user", post(add_user))
        .route("/v1/portal/create-ez", post(create_ez))
        // .route("/portal/callback", post(print_raw))
        .fallback(handler_404)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn check_authorization()-> impl IntoResponse{
    println!("\n\n\n requesting to access...\n\n\n");
    let vec_guard = AUTH_TOKEN_VECTOR.lock().await;
    if !vec_guard.is_empty(){
        // vec_guard.pop();
        Json(json!({"authorized": true}))
    }else {
        Json(json!({"authorized": false}))
    }
    
}
// sql query function store config
async fn configure_provision(Json(provision_config): Json<ProvisionConfig>) -> impl IntoResponse {
    // let provision_config: ProvisionConfig = serde_json::from_str(&value).unwrap();
    let mut vec_guard = PROVISION_VECTOR.lock().await;
    println!("{:?}", provision_config);
    *vec_guard = provision_config;
    drop(vec_guard);
    Json(json!({"response": "CONFIGURATION UPLOADED"}))
}

async fn configure_kpc(
    Path(provider): Path<String>,
    Json(provider_request): Json<ProviderRequest>,
) -> impl IntoResponse {
    println!("{:?}", provider_request);
    // let provider_request: ProviderRequest = serde_json::from_value(body).unwrap();
    let config = provider_request.configuration.to_string();
    match provider.as_str() {
        "google" => provider_record("google", provider_request.name, config).await,
        "aws" => provider_record("aws", provider_request.name, config).await,
        "azure" => provider_record("azure", provider_request.name, config).await,
        _ => println!("unknown configuration"),
    };
    // add how to response key provider id
    // add saving config with name on it
    Json(
        json!({"response": "KEY PROVIDER CONFIGURATION CREATED", "key_provider_id": Uuid::new_v4().to_string()}),
    )
}


// Used lazy static since there is no sql logic/query here that I implemented.
// it can be changed into sql query later on that fetches a table or any data required
// but for testing purposes the said crate is used.
async fn provider_record(provider: &str, name: String, config: String) {
    let kpc_id = Uuid::new_v4().to_string();
    println!("{} {}:  {}", provider, name, config);
    let mut vec_guard = PROVIDER_VECTOR.lock().await;
    let record = Providers {
        name: name,
        kpc_id: kpc_id,
        provider: provider.to_string(),
        configuration: config,
    };
    vec_guard.push(record);
    drop(vec_guard);
}

async fn return_kpcs() -> impl IntoResponse {
    let vec_guard = PROVIDER_VECTOR.lock().await;
    Json(json!(*vec_guard))
}

async fn return_users() -> impl IntoResponse {
    let vec_guard = USER_VECTOR.lock().await;
    Json(json!(*vec_guard))
}

async fn return_ez() -> impl IntoResponse {
    let vec_guard = EZ_VECTOR.lock().await;
    Json(json!(*vec_guard))
}

async fn token_request(Query(token_request): Query<GoogleTokenRequest>) -> impl IntoResponse {
    println!("Query Parameters: {:?}", token_request);
    // fetch respective provision for the tenant
    let vec_guard = PROVISION_VECTOR.lock().await;
    token_exchange(&vec_guard.client_id,
     &vec_guard.client_secret, &token_request.code, "authorization_code").await;
    print!("\n\nrequested.");
    Json(json!({"authorization": "access granted"}))
}

async fn print_raw(req: Request) -> impl IntoResponse{
    println!("Request: {:?}", req);
    let (parts, body) = req.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.unwrap();
    let body_str = String::from_utf8_lossy(&bytes);

    println!("--- REQUEST INFO ---");
    println!("Method: {}", parts.method);
    println!("URI: {}", parts.uri);
    println!("Headers: {:#?}", parts.headers);
    println!("Body:\n{}", body_str);
    "OK"
}

pub async fn token_exchange(client_id: &str, client_secret: &str, code: &str, grant_type: &str) {
    let client = Client::new();

    let token_url = "https://oauth2.googleapis.com/token";
    let redirect_uri = "http://localhost:8080/v1/portal/callback";

    let res = client
        .post(token_url)
        .form(&[
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", grant_type),
        ])
        .send()
        .await.unwrap()
        .error_for_status().unwrap(); // returns error if not 2xx
    println!("response: {:?}", res);
    println!("Status: {}", res.status());
    let bearer_token = res.text().await.unwrap();
    println!("Response Body: {}", bearer_token);
    let mut vec_guard = AUTH_TOKEN_VECTOR.lock().await;
    vec_guard.push(bearer_token);
    drop(vec_guard);
}

async fn handler_404() -> impl IntoResponse {
    Html("<h1>404 - Not Found</h1>")
}

// add user here query function here I just implement a simple global variable storing
pub async fn add_user(Json(add_user_request): Json<AddUserRequest>) -> impl IntoResponse{
    let mut vec_guard = USER_VECTOR.lock().await;
    vec_guard.push(Users {
        name: add_user_request.name.to_string(), 
        email: add_user_request.email.to_string(), 
        ez_id: add_user_request.ez_id.to_string() });
    drop(vec_guard);
    Json(
        json!({"response": "USER ADDED TO THE ZONE"})
    )
}
// returning ez_id
pub async fn create_ez(Json(create_ez_request): Json<CreateEZRequest>)-> impl IntoResponse{
    let mut vec_guard = EZ_VECTOR.lock().await;
    let ez_id_generated = Uuid::new_v4();
    vec_guard.push (EncryptionZone { 
        zone_name: create_ez_request.zone_name.to_string(), 
        ez_id: ez_id_generated.to_string(), 
        kpc_id: create_ez_request.kpc_id.to_string() });
    Json(
        json!({"response": format!("ENCRYPTION ZONE CREATED {}",ez_id_generated.to_string())})
    )
}