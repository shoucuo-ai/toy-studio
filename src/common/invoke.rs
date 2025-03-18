use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue {
    invoke(cmd, args).await
}

pub async fn invoke_for_data<P: DeserializeOwned>(cmd: &str, args: JsValue) -> Result<P, String> {
    let result: JsValue = invoke(cmd, args).await;
    let result_str = result.as_string();

    if let Some(result_str) = result_str {
        let value = serde_json::from_str::<P>(&result_str);
        match value {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Failed to convert result to string".to_string())
    }
}

pub async fn invoke_for_string(cmd: &str, args: JsValue) -> Option<String> {
    let result: JsValue = invoke(cmd, args).await;
    result.as_string()
}
