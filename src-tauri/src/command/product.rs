use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{
    command::{git_clone, uv_sync, uv_venv},
    common::{split_args, template_replace, template_replace_single},
};

use super::config::get_app_config;

lazy_static! {
    static ref APP_INSTALLED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ProductStatus {
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "nil")]
    Nil,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub icon: String,
    pub cover_image: String,
    pub package_type: String,
    pub introduction: String,
    pub service_notes: String,
    pub platforms: Vec<String>,
    pub category: String,
    pub status: Option<ProductStatus>,
    pub created_at: String,
    pub updated_at: String,
    pub device_support: DeviceSupport,
    pub requirements: Requirements,
    pub download: Download,
    pub windows: Windows,
    pub macos: Macos,
    pub linux: Linux,
    pub publisher: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DeviceSupport {
    pub cpu: bool,
    pub nvidia: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Requirements {
    pub ram: String,
    pub vram: String,
    pub disk_space: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub git_url: String,
    pub branch: String,
    pub python_version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Windows {
    pub startup: String,
    pub shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Macos {
    pub startup: String,
    pub shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Linux {
    pub startup: String,
    pub shutdown: String,
}

fn get_product_list(app_handle: &AppHandle) -> Result<Vec<Product>, String> {
    let config = get_app_config(app_handle)?;

    println!("config:{:?}", config);

    let products_dir = config.get_products_dir();
    println!("product dir:{:?}", products_dir);
    let product_files = fs::read_dir(&products_dir).map_err(|e| e.to_string())?;

    let mut products: Vec<Product> = Vec::new();

    for product_file in product_files {
        match product_file {
            Err(err) => {
                println!("product_file list error:{}", err);
            }
            Ok(product_file) => {
                println!("product_file:{}", product_file.path().to_string_lossy());
                let product = parse_product_toml(&product_file.path());
                match product {
                    Err(err) => {
                        println!("product_file parse error:{}", err);
                    }
                    Ok(mut product) => {
                        let status = if APP_INSTALLED.lock().unwrap().contains(&product.id) {
                            ProductStatus::Installed
                        } else {
                            ProductStatus::Nil
                        };
                        product.status = Some(status);
                        products.push(product);
                    }
                }
            }
        }
    }

    println!("products: {:?}", &products);
    Ok(products)
}

#[tauri::command]
pub fn get_all_product_list(app_handle: AppHandle) -> Result<String, String> {
    let products = get_product_list(&app_handle)?;
    serde_json::to_string(&products).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_installed_product_list(app_handle: AppHandle) -> Result<String, String> {
    let all_products = get_product_list(&app_handle)?;
    let installed_products: Vec<&Product> = all_products
        .iter()
        .filter(|product| APP_INSTALLED.lock().unwrap().contains(&product.id))
        .collect();
    serde_json::to_string(&installed_products).map_err(|e| e.to_string())
}

pub fn parse_product_toml(product_file: &PathBuf) -> Result<Product, String> {
    let product_toml = fs::read_to_string(product_file).map_err(|e| e.to_string())?;
    let mut product: Product = toml::from_str(&product_toml).map_err(|e| e.to_string())?;
    product.id = product_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    Ok(product)
}

#[tauri::command]
pub fn product_setup(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_file:{}", file);

    let app_config = get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);

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

    APP_INSTALLED.lock().unwrap().insert(product.id);

    Ok(())
}

#[tauri::command]
pub fn product_reinstall(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_reinstall:{}", file);

    let app_config = get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);

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

    APP_INSTALLED.lock().unwrap().insert(product.id);

    Ok(())
}

#[tauri::command]
pub fn product_uninstall(app_handle: AppHandle, file: String) -> Result<(), String> {
    println!("product_uninstall:{}", file);

    let app_config = get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;

    APP_INSTALLED.lock().unwrap().remove(&product.id);

    Ok(())
}

#[tauri::command]
pub fn product_startup(app_handle: AppHandle, file: String) -> Result<String, String> {
    println!("--------------------------------product_startup--------------------------------");
    println!("product_file:{}", file);
    let app_config = get_app_config(&app_handle)?;

    let product_dir = app_config.get_products_dir();
    let product_file = product_dir.join(file);
    println!("abs product_file:{:?}", product_file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);
    println!("product_name:{}", product_name);

    println!("product:{:?}", product);

    let install_dir = app_config.get_product_install_path().join(&product_name);

    println!("install_dir:{:?}", &install_dir);

    let output_dir = app_config.get_output_path();
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    println!("output_dir:{:?}", &output_dir);

    let startup = template_replace_single(
        &product.windows.startup,
        "output",
        &output_dir.to_string_lossy().into_owned(),
    );
    println!("startup:{}", startup);

    let mut args = split_args(&startup);
    args.insert(0, "run".to_string());
    println!("args:{:?}", args);

    let output = Command::new("uv")
        .current_dir(&install_dir)
        .args(&args)
        .output()
        .map_err(|e| e.to_string())?;

    println!("output:{:?}", &output);
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub fn get_product_name(product_id: &str) -> String {
    let name = Path::new(product_id).file_stem();
    match name {
        Some(name) => name.to_string_lossy().to_string(),
        None => product_id.to_string(),
    }
}

pub fn init_installed_products(app_handle: &AppHandle) -> Result<(), String> {
    let app_config = get_app_config(app_handle)?;

    let products_dir = app_config.get_product_install_path();
    let product_files = fs::read_dir(&products_dir).map_err(|e| e.to_string())?;

    for product_file in product_files {
        match product_file {
            Err(err) => {
                println!("product_file list error:{}", err);
            }
            Ok(product_file) => {
                let product_name = product_file.file_name();
                let mut product_name = product_name.to_string_lossy().to_string();
                product_name.push_str(".toml");
                APP_INSTALLED.lock().unwrap().insert(product_name);
            }
        }
    }
    Ok(())
}
