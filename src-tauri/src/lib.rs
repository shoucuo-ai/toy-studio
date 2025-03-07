// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_uv_cache_dir() -> Result<String, String> {
    let output = std::process::Command::new("uv")
        .arg("cache")
        .arg("dir")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let cache_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(cache_dir)
}

#[tauri::command]
fn get_python_envs() -> Result<String, String> {
    let output = std::process::Command::new("uv")
        .arg("python")
        .arg("list")
        .arg("--output-format")
        .arg("json")
        .arg("--python-preference")
        .arg("only-managed")
        .arg("--only-installed")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let envs = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    Ok(envs)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .eval("document.documentElement.lang = 'zh-CN'")
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_uv_cache_dir,
            get_python_envs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
