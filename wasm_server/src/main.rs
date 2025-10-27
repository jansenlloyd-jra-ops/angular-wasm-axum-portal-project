use axum::{
    Json, Router,
    body::to_bytes,
    extract::{Path, Request},
    response::{Html, IntoResponse},
    routing::{get_service, post},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha3::{Digest, Sha3_256};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

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
    let app = Router::new()
        // .fallback_service(ServeDir::new("assets/admin_portal"));
        .nest_service("/v1/portal", get_service(ServeDir::new("assets/admin_portal")))
        .route("/v1/kms/configuration/{config}", post(configure_kpc))
        // .route("/portal/wrapped-key", post(wrap_req))
        // .route("/portal/config", post(config_handler))
        // .route("/portal/callback", post(print_raw))
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn configure_kpc(Path(config): Path<String>, Json(body): Json<Value>) -> impl IntoResponse {
    match config.as_str() {
        "google" => println!("google: {:?}", body),
        "aws" => println!("aws: {:?}", body),
        "azure" => println!("azure: {:?}", body),
        _ => println!("unknown configuration"),
    };
    Json(json!({"response": "KEY PROVIDER CONFIGURATION CREATED"}))
}

async fn handler_404() -> impl IntoResponse {
    Html("<h1>404 - Not Found</h1>")
}

async fn wrap_req(Json(value): Json<Value>) -> impl IntoResponse {
    println!("{:?}", value);
    let req_value = value.get("key").unwrap().as_str().unwrap();
    let mut hasher = Sha3_256::new();
    hasher.update(req_value);
    let res = WrapResp {
        wrapped_key: URL_SAFE_NO_PAD.encode(hasher.finalize()),
        msg_id: Some(make_opaque(32).await),
    };
    Json(res)
}

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
