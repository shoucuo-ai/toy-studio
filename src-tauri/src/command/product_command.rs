use std::{
    collections::HashMap,
    fs::{self},
    process::Child,
    sync::{Arc, Mutex},
};

use tauri::AppHandle;

use crate::{
    get_file_name_without_suffix, git_clone, split_args, uv_sync, uv_venv, Product,
};

use crate::AppConfig;

lazy_static! {
    pub static ref APP_INSTALLED: Mutex<HashMap<String, Option<Arc<Mutex<Child>>>>> =
        Mutex::new(HashMap::new());
}

#[tauri::command]
pub fn get_all_product_list(app_handle: AppHandle) -> Result<String, String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let products = app_config.get_product_list()?;
    serde_json::to_string(&products).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_installed_product_list(app_handle: AppHandle) -> Result<String, String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let all_products = app_config.get_product_list()?;
    let installed_products: Vec<&Product> = all_products
        .iter()
        .filter(|product| APP_INSTALLED.lock().unwrap().contains_key(&product.id))
        .collect();
    serde_json::to_string(&installed_products).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn product_setup(app_handle: AppHandle, product_id: String) -> Result<(), String> {
    println!("product_id:{}", product_id);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(product_id);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    println!("install_dir:{:?}", &install_dir);

    let git_url = product.download.git_url;
    let branch = product.download.branch;
    let bak_dir = app_config.get_product_bak_path();

    git_clone(git_url, branch, &install_dir, &bak_dir)?;
    uv_venv(&install_dir, &product.download.python_version)?;
    uv_sync(&install_dir)?;

    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

#[tauri::command]
pub fn product_reinstall(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_reinstall:{}", file);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    println!("install_dir:{:?}", &install_dir);

    let git_url = product.download.git_url;
    let branch = product.download.branch;
    let bak_dir = app_config.get_product_bak_path();

    git_clone(git_url, branch, &install_dir, &bak_dir)?;
    uv_venv(&install_dir, &product.download.python_version)?;
    uv_sync(&install_dir)?;

    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

#[tauri::command]
pub fn product_uninstall(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_uninstall:{}", file);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;

    APP_INSTALLED.lock().unwrap().remove(&product.id);

    Ok(())
}

#[tauri::command]
pub fn product_startup(app_handle: AppHandle, product_id: String) -> Result<(), String> {
    println!("--------------------------------product_startup--------------------------------");
    println!("product_id:{}", product_id);

    // 1. get product info by product_id
    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(product_id);
    println!("abs product_file:{:?}", product_file);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);
    println!("product_name:{}", product_name);

    println!("product:{:?}", product);

    // 2. check if product is already running
    if let Some(Some(child)) = APP_INSTALLED.lock().unwrap().get(&product.id) {
        if let Ok(None) = child.lock().unwrap().try_wait() {
            return Err("Product already running".to_string());
        }
    }

    // 3. create install_dir
    let install_dir = app_config.get_product_install_path().join(&product_name);
    println!("install_dir:{:?}", &install_dir);

    // 4. create output_dir & startup command
    let output_dir = app_config.get_output_path();
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    println!("output_dir:{:?}", &output_dir);

    let startup = product.get_startup_command(&output_dir)?;
    println!("startup:{}", startup);

    // 5. run startup command
    let mut args = split_args(&startup);
    args.insert(0, "run".to_string());
    println!("args:{:?}", args);
    crate::run_command(&install_dir, "uv", &args, &product_name, &product.id)
}

#[tauri::command]
pub fn product_shutdown(file: String) -> Result<(), String> {
    println!("product_shutdown:{}", file);

    let mut child = APP_INSTALLED.lock().unwrap().remove(&file);
    if let Some(Some(child)) = child.take() {
        child.lock().unwrap().kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn product_upgrade(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_reinstall:{}", file);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_file = app_config.get_product_dir(&file);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    println!("install_dir:{:?}", &install_dir);

    let git_url = product.download.git_url;
    let branch = product.download.branch;
    let bak_dir = app_config.get_product_bak_path();

    git_clone(git_url, branch, &install_dir, &bak_dir)?;
    uv_venv(&install_dir, &product.download.python_version)?;
    uv_sync(&install_dir)?;

    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

pub(crate) fn init_installed_products(app_handle: &AppHandle) -> Result<(), String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;

    let products_dir = app_config.get_product_install_path();
    let product_files = fs::read_dir(&products_dir).map_err(|e| e.to_string())?;

    for product_file in product_files {
        match product_file {
            Err(err) => {
                println!("product_file list error:{}", err);
            }
            Ok(product_file) => {
                let product_name = product_file.file_name();
                let mut product_id = product_name.to_string_lossy().to_string();
                product_id.push_str(".toml");
                APP_INSTALLED.lock().unwrap().insert(product_id, None);
            }
        }
    }
    Ok(())
}
