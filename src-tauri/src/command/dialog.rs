use std::{path::PathBuf, process::Command};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::{
    command::{get_app_config, get_product_name, parse_product_toml},
    common::is_wsl,
};

#[tauri::command]
pub async fn select_directory(app_handle: AppHandle) -> Result<String, String> {
    let f = app_handle.dialog().file();
    let f = f
        .set_directory(PathBuf::from("."))
        .set_title("select directory");
    let dir = f.blocking_pick_folder();
    match dir {
        Some(path) => {
            let path = path.as_path().unwrap().to_string_lossy().into_owned();
            println!("path: {:?}", path);
            Ok(path)
        }
        None => Ok("".to_string()),
    }
}

#[tauri::command]
pub async fn open_directory(dir: &str) -> Result<(), String> {
    let res = std::fs::create_dir_all(&dir);
    println!("res: {:?}", res);
    println!("open_directory: {:?}", dir);
    if is_wsl() {
        wsl_open_in_explorer(dir);
    } else {
        showfile::show_path_in_file_manager(dir);
    }
    Ok(())
}

#[tauri::command]
pub async fn open_product_directory(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_file:{}", file);

    let app_config = get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = PathBuf::from(product_dir).join(file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);
    println!("install_dir: {:?}", install_dir);
    if is_wsl() {
        wsl_open_in_explorer(&install_dir.to_string_lossy().into_owned());
    } else {
        showfile::show_path_in_file_manager(install_dir);
    }
    Ok(())
}

pub fn wsl_open_in_explorer(path: &str) {
    // 如果在 WSL 中运行，则调用 explorer.exe 打开 Windows 资源管理器
    let path = path.replace("/", "\\");
    println!("path: {}", path);
    let path = format!(r"\\wsl.localhost\Ubuntu-24.04{}", path); // 确保路径格式适合 Windows
    println!("explorer path: {}", path);
    let _x = Command::new("explorer.exe").arg(&path).spawn();
}
