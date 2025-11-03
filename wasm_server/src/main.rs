use axum::{
    Json, Router,
    extract::Path,
    http::request,
    response::{Html, IntoResponse},
    routing::{get_service, post},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use sha3::{Digest, Sha3_256};
use lazy_static::lazy_static; // or once_cell::sync::OnceCell
use std::sync::Mutex;
use std::{net::SocketAddr, vec};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Providers {
    name: String,
    provider: String,
    kpc_id: String,
    configuration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Users{
    name: String,
    email: String,
    ez_id: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EncryptionZone{
    zone_name: String,
    ez_id: String,
    kpc_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderRequest {
    name: String,
    configuration: Value,
}
lazy_static! {
    pub static ref PROVIDER_VECTOR: Mutex<Vec<Providers>> = Mutex::new(vec![
        Providers {
            name: "test_department1_key".to_string(),
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
            name: "test_department2_key".to_string(),
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
            name: "test_department3_key".to_string(),
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
            name: "test_department4_key".to_string(),
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
        .route("/v1/portal/provision-configuration", post(config_handler))
        .route("/v1/portal/kpc-fetch", post(return_kpcs))
        .route("/v1/portal/users", post(return_users))
        // .route("/portal/callback", post(print_raw))
        .fallback(handler_404)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn config_handler(Json(value): Json<Value>) -> impl IntoResponse {
    println!("{:?}", value);
    Json(json!({"response": "CONFIGURATION UPLOADED"}))
}

async fn configure_kpc(
    Path(provider): Path<String>,
    Json(request): Json<ProviderRequest>,
) -> impl IntoResponse {
    println!("{:?}", request);
    // let request: ProviderRequest = serde_json::from_value(body).unwrap();
    let config = request.configuration.to_string();
    match provider.as_str() {
        "google" => provider_record("google", request.name, config).await,
        "aws" => provider_record("aws", request.name, config).await,
        "azure" => provider_record("azure", request.name, config).await,
        _ => println!("unknown configuration"),
    };
    // add how to response key provider id
    // add saving config with name on it
    Json(
        json!({"response": "KEY PROVIDER CONFIGURATION CREATED", "key_provider_id": Uuid::new_v4().to_string()}),
    )
}

async fn handler_404() -> impl IntoResponse {
    Html("<h1>404 - Not Found</h1>")
}
// Used lazy static since there is no sql logic/query here that I implemented.
// it can be changed into sql query later on that fetches a table or any data required
// but for testing purposes the said crate is used.
async fn provider_record(provider: &str, name: String, config: String) {
    let kpc_id = Uuid::new_v4().to_string();
    println!("{} {}:  {}", provider, name, config);
    let mut vec_guard = PROVIDER_VECTOR.lock().unwrap();
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
    let mut vec_guard = PROVIDER_VECTOR.lock().unwrap();
    Json(json!(*vec_guard))
}

async fn return_users() -> impl IntoResponse {
    let mut vec_guard = USER_VECTOR.lock().unwrap();
    Json(json!(*vec_guard))
}

// pub async fn print_raw(req: Request) -> impl IntoResponse {
//     let (parts, body) = req.into_parts();
//     let bytes = to_bytes(body, usize::MAX).await.unwrap();
//     let body_str = String::from_utf8_lossy(&bytes);

//     println!("--- REQUEST INFO ---");
//     println!("Method: {}", parts.method);
//     println!("URI: {}", parts.uri);
//     println!("Headers: {:#?}", parts.headers);
//     println!("Body:\n{}", body_str);

//     "OK"
// }
