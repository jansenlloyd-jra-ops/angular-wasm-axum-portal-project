use axum::{
    Json, Router, extract::Path, http::request, response::{Html, IntoResponse}, routing::{get_service, post}
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use sha3::{Digest, Sha3_256};
use lazy_static::lazy_static; // or once_cell::sync::OnceCell
use std::{net::SocketAddr, vec};
use std::sync::Mutex;
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
pub struct ProviderRequest {
    name: String,
    configuration: Value,
}

lazy_static! {
    static ref PROVIDER_VECTOR: Mutex<Vec<Providers>> = Mutex::new(Vec::new());
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
        // .route("/portal/callback", post(print_raw))
        .fallback(handler_404)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn configure_kpc(
    Path(provider): Path<String>,
    Json(request): Json<ProviderRequest>,
) -> impl IntoResponse {
    println!("{:?}", request);
    // let request: ProviderRequest = serde_json::from_value(body).unwrap();
    let kpc_id_generated = Uuid::new_v4().to_string();
    let config = request.configuration.to_string();
    match provider.as_str() {
        "google" => provider_record("google", request.name, kpc_id_generated, config).await,
        "aws" => provider_record("aws", request.name, kpc_id_generated, config).await,
        "azure" => provider_record("azure", request.name, kpc_id_generated, config).await,
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

async fn provider_record(provider: &str, name: String, kpc_id: String, config: String) {
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

async fn return_kpcs() -> impl IntoResponse{
    let mut vec_guard = PROVIDER_VECTOR.lock().unwrap();
    Json(json!(*vec_guard))
}

// async fn wrap_req(Json(value): Json<Value>) -> impl IntoResponse {
//     println!("{:?}", value);
//     let req_value = value.get("key").unwrap().as_str().unwrap();
//     let mut hasher = Sha3_256::new();
//     hasher.update(req_value);
//     let res = WrapResp {
//         wrapped_key: URL_SAFE_NO_PAD.encode(hasher.finalize()),
//         msg_id: Some(make_opaque(32).await),
//     };
//     Json(res)
// }

async fn config_handler(Json(value): Json<Value>) -> impl IntoResponse {
    println!("{:?}", value);
    Json(json!({"response": "CONFIGURATION UPLOADED"}))
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
