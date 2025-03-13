// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
mod command;
mod common;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .eval("document.documentElement.lang = 'zh-CN'")
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            command::uv::uv_get_cache_dir,
            command::uv::uv_get_python_envs,
            command::git::git_clone,
            command::app::get_app_list,
            command::config::get_config,
            command::config::set_config,
            command::dialog::select_directory,
            command::dialog::open_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
