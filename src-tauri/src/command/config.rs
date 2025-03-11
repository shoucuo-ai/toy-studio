use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub project_root_dir: String,
    pub enable_external_uv: bool,
}

impl AppConfig {
    fn default(app_handle: &AppHandle) -> Self {
        let dir = app_handle.path().app_data_dir();
        let dir = dir.unwrap_or_else(|_| PathBuf::from(""));
        let dir = dir.to_string_lossy();
        Self {
            language: "zh".to_string(),
            project_root_dir: dir.to_string(),
            enable_external_uv: true,
        }
    }
}

fn get_config_file_path(app_handle: &AppHandle) -> PathBuf {
    let config_dir = app_handle.path().app_config_dir().unwrap();
    fs::create_dir_all(&config_dir).unwrap();
    let dist = config_dir.join("config.json");
    println!("config_dir: {:?}", dist);
    dist
}

#[tauri::command]
pub fn get_config(app_handle: AppHandle) -> Result<String, String> {
    let config_path = get_config_file_path(&app_handle);
    if !config_path.exists() {
        let default_config = AppConfig::default(&app_handle);
        let config_str =
            serde_json::to_string_pretty(&default_config).map_err(|e| e.to_string())?;
        fs::write(&config_path, &config_str).map_err(|e| e.to_string())?;
        return Ok(config_str);
    }

    fs::read_to_string(&config_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(app_handle: AppHandle, config: String) -> Result<String, String> {
    let config_path = get_config_file_path(&app_handle);

    // 验证配置格式是否正确
    serde_json::from_str::<AppConfig>(&config).map_err(|e| e.to_string())?;

    fs::write(&config_path, &config).map_err(|e| e.to_string())?;

    Ok(config)
}
