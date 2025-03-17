use std::fs;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::config::get_app_config;

#[derive(Deserialize, Serialize)]
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
    status: String,
    created_at: String,
    updated_at: String,
    device_support: DeviceSupport,
    requirements: Requirements,
    download: Download,
    command: Command,
}

#[derive(Deserialize, Serialize)]
struct DeviceSupport {
    cpu: bool,
    nvidia: bool,
}

#[derive(Deserialize, Serialize)]
struct Requirements {
    ram: String,
    vram: String,
    disk_space: String,
}

#[derive(Deserialize, Serialize)]
struct Download {
    git_url: String,
    branch: String,
    setup_instructions: String,
}

#[derive(Deserialize, Serialize)]
struct Command {
    start: String,
}

#[tauri::command]
pub fn get_product_list(app_handle: AppHandle) -> Result<String, String> {
    let config = get_app_config(&app_handle)?;

    let products_dir = config.get_products_dir();
    let products = fs::read_dir(&products_dir).map_err(|e| e.to_string())?;
    let products = products
        .map(|product| product.unwrap().path().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    let products = products
        .iter()
        .map(|product| parse_product_toml(product))
        .filter(|product| product.is_ok())
        .map(|product| product.unwrap())
        .collect::<Vec<Product>>();

    serde_json::to_string(&products).map_err(|e| e.to_string())
}

fn parse_product_toml(product_file: &str) -> Result<Product, String> {
    let product_toml = fs::read_to_string(product_file).map_err(|e| e.to_string())?;
    let product: Product = toml::from_str(&product_toml).map_err(|e| e.to_string())?;
    Ok(product)
}


#[tauri::command]
pub fn product_setup(app_handle: AppHandle, product_file: &str) -> Result<String, String> {

    Ok(String::from("success"))
}