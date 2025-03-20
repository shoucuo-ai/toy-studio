use std::fs;
use tauri::AppHandle;

use crate::AppConfig;

#[tauri::command]
pub fn get_config(app_handle: AppHandle) -> Result<String, String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let json = serde_json::to_string_pretty(&app_config);
    json.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(app_handle: AppHandle, config: String) -> Result<String, String> {
    let config_path = AppConfig::get_config_file_path(&app_handle);

    // 验证配置格式是否正确
    serde_json::from_str::<AppConfig>(&config).map_err(|e| e.to_string())?;

    fs::write(&config_path, &config).map_err(|e| e.to_string())?;

    Ok(config)
}
