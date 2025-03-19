use command::init_installed_products;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod common;
#[macro_use]
extern crate lazy_static;

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
            let app_handle = app.handle();
            let _ = init_installed_products(&app_handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            command::uv_get_cache_dir,
            command::uv_get_python_envs,
            command::get_all_product_list,
            command::get_config,
            command::set_config,
            command::select_directory,
            command::open_directory,
            command::product_setup,
            command::get_installed_product_list,
            command::product_startup,
            command::open_product_directory,
            command::product_reinstall,
            command::product_uninstall,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
