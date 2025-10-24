use reqwest_wasm::Client;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub async fn wrapped_key_request(key: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();
    let window = window().ok_or("no window")?;
    let location = window.location();
    let origin = location
        .origin()
        .map_err(|_| JsValue::from_str("no origin"))?;

    let req: Value = json!({ "key": key });
    let res = client
        .post(format!("{}/portal/wrapped-key", origin))
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
pub async fn send_config(config: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();
    let window = window().ok_or("no window")?;
    let location = window.location();
    let origin = location
        .origin()
        .map_err(|_| JsValue::from_str("no origin"))?;

    let req: Value = serde_json::to_value(config).unwrap();

    let res = client
        .post(format!("{}/portal/config", origin))
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
