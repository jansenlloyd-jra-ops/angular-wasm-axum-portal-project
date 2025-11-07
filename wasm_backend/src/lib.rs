use reqwest_wasm::Client;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
// use web_sys::window;

const ORIGIN: &str = "http://127.0.0.1:8080";

#[wasm_bindgen]
pub async fn request_provision(config: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();

    let req: Value = serde_json::from_str(config)
        .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

    let res = client
        .post(format!("{}/v1/portal/provision-configuration", ORIGIN))
        .json(&req)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}

#[wasm_bindgen]
pub async fn request_create_kpc(name: &str, config: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();

    let req: Value = serde_json::from_str(config)
        .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

    let res = client
        .post(format!("{}/v1/portal/key-provider/{}", ORIGIN, name))
        .json(&req)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}

#[wasm_bindgen]
pub async fn fetch_all_kpc() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/kpc", ORIGIN))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}


#[wasm_bindgen]
pub async fn fetch_all_user() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/users", ORIGIN))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}

#[wasm_bindgen]
pub async fn fetch_all_ez() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/ez", ORIGIN))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}

#[wasm_bindgen]
pub async fn request_authenticate() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/authorization/sign-in", ORIGIN))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}

#[wasm_bindgen]
pub async fn request_log_out() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/authorization/sign-out", ORIGIN))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}



#[wasm_bindgen]
pub async fn request_add_user(name: &str, email: &str, ez_id: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/add-user", ORIGIN))
        .json(&json!({"name": name, "email": email, "ez_id": ez_id}))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}


#[wasm_bindgen]
pub async fn request_create_ez(zone_name: &str, key_name: &str, kpc_id: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/create-ez", ORIGIN))
        .json(&json!({"zone_name": zone_name, "key_name": key_name, "kpc_id": kpc_id}))
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let res_req = res
        .text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&res_req))
}
