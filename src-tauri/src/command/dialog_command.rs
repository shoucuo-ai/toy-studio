use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::{get_file_name_without_suffix, is_wsl, AppConfig};

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

/// 打开系统目录（ToyStudio 系统 自己的目录）
#[tauri::command]
pub async fn open_system_directory(app_handle: AppHandle, dir: &str) -> Result<(), String> {
    println!("------------------open_system_directory--------------------------------");
    println!("open_system_directory: {:?}", dir);
    let dist = match dir {
        "data" => Some(app_handle.path().app_data_dir()),
        "cache" => Some(app_handle.path().app_cache_dir()),
        "config" => Some(app_handle.path().app_config_dir()),
        "local_data" => Some(app_handle.path().app_local_data_dir()),
        "log" => Some(app_handle.path().app_log_dir()),
        _ => None,
    };
    println!("dist: {:?}", dist);
    match dist {
        Some(Ok(dist)) => {
            show_path_in_file_manager(&dist.to_string_lossy().into_owned());
            Ok(())
        }
        _ => Err("invalid directory".to_string()),
    }
}

/// 打开托管目录（ToyStudio中用于托管产品的目录）
#[tauri::command]
pub async fn open_managed_directory(app_handle: AppHandle, dir: &str) -> Result<(), String> {
    println!("------------------open_managed_directory--------------------------------");
    println!("dir: {:?}", dir);
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let project_root_dir = app_config.project_root_dir.clone();
    println!("project_root_dir: {}", project_root_dir);

    let full_path = PathBuf::from(project_root_dir).join(dir);
    println!("full_path: {}", full_path.display());

    let _ = std::fs::create_dir_all(&full_path);

    show_path_in_file_manager(&full_path);

    Ok(())
}

/// 打开产品目录（ToyStudio中托管的产品目录）
#[tauri::command]
pub async fn open_product_directory(
    app_handle: AppHandle,
    product_id: String,
) -> Result<(), String> {
    println!("product_id:{}", product_id);

    let app_config = AppConfig::get_app_config(&app_handle)?;
    let product_name = get_file_name_without_suffix(&product_id);
    let install_dir = app_config.get_product_install_path().join(&product_name);

    println!(
        "install_dir: {}, product_name: {}",
        install_dir.to_string_lossy().into_owned(),
        product_name
    );

    show_path_in_file_manager(&install_dir);

    Ok(())
}

fn show_path_in_file_manager<P: AsRef<Path>>(path: P) {
    println!("---------------------show_path_in_file_manager--------------------------------");
    let path = path.as_ref().to_string_lossy().into_owned();
    match std::env::consts::OS {
        "windows" => windows_open_in_explorer(&path),
        "linux" => {
            if is_wsl() {
                wsl_open_in_explorer(&path);
            } else {
                linux_open_in_explorer(&path);
            }
        }
        _ => (),
    }
}

pub fn windows_open_in_explorer(path: &str) {
    // 如果在 WSL 中运行，则调用 explorer.exe 打开 Windows 资源管理器
    let path = path.replace("/", "\\");
    println!("path: {}", path);
    let _x = Command::new("explorer.exe").arg(&path).spawn();
}

pub fn wsl_open_in_explorer(path: &str) {
    // 如果在 WSL 中运行，则调用 explorer.exe 打开 Windows 资源管理器
    let path = path.replace("/", "\\");
    println!("path: {}", path);
    let path = format!(r"\\wsl.localhost\Ubuntu-24.04{}", path); // 确保路径格式适合 Windows
    println!("explorer path: {}", path);
    let _x = Command::new("explorer.exe").arg(&path).spawn();
}

pub fn linux_open_in_explorer(path: &str) {
    let path = path.replace("/", "\\");
    println!("path: {}", path);
    let _x = Command::new("explorer.exe").arg(&path).spawn();
}
