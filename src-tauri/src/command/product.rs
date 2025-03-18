use std::{
    collections::HashSet,
    fs::{self},
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::command::{get_product_bak_path, get_product_install_path, git_clone, uv_sync, uv_venv};

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
struct Product {
    id: String,
    name: String,
    version: String,
    description: String,
    icon: String,
    cover_image: String,
    package_type: String,
    introduction: String,
    service_notes: String,
    platforms: Vec<String>,
    category: String,
    status: Option<ProductStatus>,
    created_at: String,
    updated_at: String,
    device_support: DeviceSupport,
    requirements: Requirements,
    download: Download,
    windows: Windows,
    macos: Macos,
    linux: Linux,
    publisher: Option<String>,
    file_size: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct DeviceSupport {
    cpu: bool,
    nvidia: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Requirements {
    ram: String,
    vram: String,
    disk_space: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Download {
    git_url: String,
    branch: String,
    python_version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Windows {
    startup: String,
    shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Macos {
    startup: String,
    shutdown: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
struct Linux {
    startup: String,
    shutdown: String,
}

fn get_product_list(app_handle: &AppHandle) -> Result<Vec<Product>, String> {
    let config = get_app_config(app_handle)?;

    println!("config:{:?}", config);

    let products_dir = config.get_products_dir();
    println!("product dir:{}", products_dir);
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

fn parse_product_toml(product_file: &PathBuf) -> Result<Product, String> {
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

    let product_dir = get_app_config(&app_handle)?.get_products_dir();
    let product_file = PathBuf::from(product_dir).join(file);
    let product = parse_product_toml(&product_file)?;
    let product_name = get_product_name(&product.id);

    println!("product:{:?}", product);

    let install_dir = get_product_install_path(&app_handle).join(&product_name);

    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    println!("install_dir:{:?}", &install_dir);

    let git_url = product.download.git_url;
    let branch = product.download.branch;
    let bak_dir = get_product_bak_path(&app_handle);

    git_clone(git_url, branch, &install_dir, &bak_dir)?;
    uv_venv(&install_dir, &product.download.python_version)?;
    uv_sync(&install_dir)?;

    APP_INSTALLED.lock().unwrap().insert(product.id);

    Ok(())
}

fn get_product_name(product_id: &str) -> String {
    let name = Path::new(product_id).file_stem();
    match name {
        Some(name) => name.to_string_lossy().to_string(),
        None => product_id.to_string(),
    }
}

pub fn init_installed_products(app_handle: &AppHandle) -> Result<(), String> {
    let products_dir = get_product_install_path(app_handle);
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
