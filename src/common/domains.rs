use sycamore::prelude::*;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{invoke_for_data, invoke_tauri};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub language: String,
    pub project_root_dir: String,
    pub enable_external_uv: bool,
    pub uv_cache_dir: String,
    pub dev_mode: Option<bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            project_root_dir: "./".to_string(),
            enable_external_uv: false,
            uv_cache_dir: "./cache".to_string(),
            dev_mode: Some(false),
        }
    }
}

impl AppConfig {
    pub async fn load() -> Result<Self, String> {
        invoke_for_data::<AppConfig>("get_config", JsValue::NULL).await
    }

    pub async fn save(&self) -> Result<(), String> {
        console_log!("AppConfig save: {:?}", self);
        let config_str = serde_json::to_string(self).map_err(|e| e.to_string())?;
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "config": config_str
        }))
        .map_err(|e| e.to_string())?;
        match invoke_tauri("set_config", args).await {
            _ => Ok(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub icon: String,
    pub cover_image: String,
    pub package_type: String,
    pub introduction: String,
    pub service_notes: String,
    pub platforms: Vec<String>,
    pub category: String,
    pub install: Option<bool>,
    pub running: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
    pub device_support: DeviceSupport,
    pub requirements: Requirements,
    pub download: Download,
    pub windows: Windows,
    pub macos: Macos,
    pub linux: Linux,
    pub publisher: Option<String>,
    pub file_size: Option<i64>,
}

impl Product {
    pub async fn load_all_products() -> Result<Vec<Product>, String> {
        invoke_for_data::<Vec<Product>>("get_meta_product_list", JsValue::NULL).await
    }

    pub async fn load_installed_products() -> Result<Vec<Product>, String> {
        invoke_for_data::<Vec<Product>>("get_installed_product_list", JsValue::NULL).await
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DeviceSupport {
    pub cpu: bool,
    pub nvidia: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Requirements {
    pub ram: String,
    pub vram: String,
    pub disk_space: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub git_url: String,
    pub branch: String,
    pub python_version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Windows {
    pub startup: String,
    pub shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Macos {
    pub startup: String,
    pub shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Linux {
    pub startup: String,
    pub shutdown: String,
}
