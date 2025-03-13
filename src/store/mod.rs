use serde::{Deserialize, Serialize};
use serde_json;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub language: String,
    pub project_root_dir: String,
    pub enable_external_uv: bool,
    pub uv_cache_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            project_root_dir: "./".to_string(),
            enable_external_uv: false,
            uv_cache_dir: "./cache".to_string(),
        }
    }
}

impl AppConfig {
    pub async fn load() -> Result<Self, String> {
        let value = invoke("get_config", JsValue::NULL).await;
        if let Some(value) = value.as_string() {
            console_log!("AppConfig load value: {}", value);
            let config = serde_json::from_str::<AppConfig>(&value).map_err(|e| e.to_string())?;
            Ok(config)
        } else {
            Err("Failed to load config".to_string())
        }
    }

    pub async fn save(&self) -> Result<(), String> {
        console_log!("AppConfig save: {:?}", self);
        let config_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "config": config_str
        }))
        .map_err(|e| e.to_string())?;
        match invoke("set_config", args).await {
            _ => Ok(()),
        }
    }
}
