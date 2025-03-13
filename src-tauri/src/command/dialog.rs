use std::{path::PathBuf, process::Command};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::common::is_wsl;

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

pub fn wsl_open_in_explorer(path: &str) {
    // 如果在 WSL 中运行，则调用 explorer.exe 打开 Windows 资源管理器
    let path = path.replace("/", "\\");
    println!("path: {}", path);
    let path = format!(r"\\wsl.localhost\Ubuntu-24.04{}", path); // 确保路径格式适合 Windows
    println!("explorer path: {}", path);
    let _x = Command::new("explorer.exe").arg(&path).spawn();
}
