use serde::{Deserialize, Serialize};
use serde_json;
use serde_wasm_bindgen::{from_value, to_value};
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
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            project_root_dir: "".to_string(),
            enable_external_uv: false,
        }
    }
}

impl AppConfig {
    pub async fn load() -> Result<Self, String> {
        let value = invoke("get_config", to_value(&()).unwrap()).await;

        let value = from_value::<String>(value).map_err(|e| e.to_string())?;

        let config = serde_json::from_str::<AppConfig>(&value).map_err(|e| e.to_string())?;
        Ok(config)
    }

    pub async fn save(&self) -> Result<(), String> {
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
