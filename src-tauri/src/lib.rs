// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod common;
mod cross;
pub use command::*;
pub use common::*;
pub use cross::*;

#[macro_use]
extern crate lazy_static;

use crate::init_installed_products;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("test.log".to_string()),
                    }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();
            let _ = init_installed_products(&app_handle);
            let _ = init_meta_products(&app_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::uv_get_cache_dir,
            command::uv_get_python_envs,
            command::set_config,
            command::get_config,
            command::get_meta_product_list,
            command::get_installed_product_list,
            command::select_directory,
            command::open_system_directory,
            command::open_managed_directory,
            command::open_product_directory,
            command::product_install,
            command::product_reinstall,
            command::product_uninstall,
            command::product_startup,
            command::product_shutdown,
            command::product_upgrade,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
