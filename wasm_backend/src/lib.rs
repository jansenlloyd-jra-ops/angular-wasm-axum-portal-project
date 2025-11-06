use reqwest_wasm::Client;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
// use web_sys::window;

const ORIGIN: &str = "http://127.0.0.1:8080";

#[wasm_bindgen]
pub async fn provision_config_request(config: &str) -> Result<JsValue, JsValue> {
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
pub async fn kpc_config_request(name: &str, config: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();

    let req: Value = serde_json::from_str(config)
        .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;

    let res = client
        .post(format!("{}/v1/portal/kms-configuration/{}", ORIGIN, name))
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
pub async fn kpc_fetch_all() -> Result<JsValue, JsValue> {
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
pub async fn users_fetch_all() -> Result<JsValue, JsValue> {
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
pub async fn ez_fetch_all() -> Result<JsValue, JsValue> {
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
pub async fn is_authenticated() -> Result<JsValue, JsValue> {
    let client = Client::new();

    let res = client
        .post(format!("{}/v1/portal/authorization", ORIGIN))
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
pub async fn add_user_request(name: &str, email: &str, ez_id: &str) -> Result<JsValue, JsValue> {
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
