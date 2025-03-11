// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod app;
mod git;
mod uv;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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
            uv::uv_cache::get_uv_cache_dir,
            uv::uv_envs::get_python_envs,
            git::git_clone::git_clone,
            app::app_list::get_app_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
