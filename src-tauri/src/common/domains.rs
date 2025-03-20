use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{uv_get_cache_dir, APP_INSTALLED};

use super::template_replace_single;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub project_root_dir: String,
    pub enable_external_uv: bool,
    pub uv_cache_dir: String,
}

impl AppConfig {
    pub fn default(app_handle: &AppHandle) -> Self {
        let dir = app_handle.path().app_data_dir();
        let dir = dir.unwrap_or_else(|_| PathBuf::from(""));
        let dir = dir.to_string_lossy();
        let cache_dir = uv_get_cache_dir().unwrap_or_else(|_| "".to_string());
        Self {
            language: "zh".to_string(),
            project_root_dir: dir.to_string(),
            enable_external_uv: true,
            uv_cache_dir: cache_dir,
        }
    }

    pub fn get_products_dir(&self) -> PathBuf {
        let dir = PathBuf::from(&self.project_root_dir);
        let dir = dir.join("./.local/products");
        dir
    }

    pub fn get_product_dir(&self, product_id: &str) -> PathBuf {
        let dir = PathBuf::from(&self.project_root_dir);
        let dir = dir.join("./.local/products");
        let dir = dir.join(product_id);
        dir
    }

    pub fn get_product_install_path(&self) -> PathBuf {
        let dir = PathBuf::from(&self.project_root_dir);
        let dir = dir.join("./apps");
        dir
    }

    pub fn get_product_bak_path(&self) -> PathBuf {
        let dir = PathBuf::from(&self.project_root_dir);
        let dir = dir.join("./.local/bak");
        dir
    }

    pub fn get_output_path(&self) -> PathBuf {
        let dir = PathBuf::from(&self.project_root_dir);
        let dir = dir.join("./output");
        dir
    }

    pub fn get_config_file_path(app_handle: &AppHandle) -> PathBuf {
        let config_dir = app_handle.path().app_config_dir().unwrap();
        fs::create_dir_all(&config_dir).unwrap();
        let dist = config_dir.join("config.json");
        println!("config_dir: {:?}", dist);
        dist
    }

    pub fn get_app_config(app_handle: &AppHandle) -> Result<AppConfig, String> {
        let config_path = Self::get_config_file_path(&app_handle);
        if !config_path.exists() {
            let app_config = AppConfig::default(&app_handle);
            let config_str =
                serde_json::to_string_pretty(&app_config).map_err(|e| e.to_string())?;
            fs::write(&config_path, &config_str).map_err(|e| e.to_string())?;
            return Ok(app_config);
        }
        match fs::read_to_string(&config_path) {
            Ok(json) => {
                let app_config = serde_json::from_str::<AppConfig>(&json);
                app_config.map_err(|e| e.to_string())
            }
            Err(err) => return Err(err.to_string()),
        }
    }
    pub fn get_product_list(&self) -> Result<Vec<Product>, String> {
        println!("config:{:?}", self);

        let products_dir = self.get_products_dir();
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
                    let product = Product::parse_product_toml(&product_file.path());
                    match product {
                        Err(err) => {
                            println!("product_file parse error:{}", err);
                        }
                        Ok(mut product) => {
                            if let Some(x) = APP_INSTALLED.lock().unwrap().get(&product.id) {
                                product.install = Some(true);
                                if let Some(child) = x {
                                    if let Ok(None) = child.lock().unwrap().try_wait() {
                                        product.running = Some(true);
                                    } else {
                                        product.running = None;
                                    }
                                } else {
                                    product.running = None;
                                }
                            } else {
                                product.install = None;
                                product.running = None;
                            }
                            products.push(product);
                        }
                    }
                }
            }
        }

        println!("products: {:?}", &products);
        Ok(products)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Product {
    /// 产品ID: 唯一标识, 使用产品配置`*.toml` 文件名作为ID。
    pub id: String,
    /// 产品名称
    pub name: String,
    /// 产品版本
    pub version: String,
    /// 产品描述
    pub description: String,
    /// 产品图标
    pub icon: String,
    /// 产品封面图片
    pub cover_image: String,
    /// 产品类型
    pub package_type: String,
    /// 产品介绍
    pub introduction: String,
    /// 产品服务说明
    pub service_notes: String,
    /// 产品支持平台
    pub platforms: Vec<String>,
    /// 产品分类
    pub category: String,
    /// 产品安装状态
    pub install: Option<bool>,
    /// 产品运行状态
    pub running: Option<bool>,
    /// 产品创建时间
    pub created_at: String,
    /// 产品更新时间
    pub updated_at: String,
    /// 产品支持设备
    pub device_support: DeviceSupport,
    /// 产品需求
    pub requirements: Requirements,
    /// 产品下载
    pub download: Download,
    /// 产品Windows启动命令
    pub windows: Windows,
    /// 产品MacOS启动命令
    pub macos: Macos,
    /// 产品Linux启动命令
    pub linux: Linux,
    /// 产品发布者
    pub publisher: Option<String>,
    /// 产品文件大小
    pub file_size: Option<i64>,
}

impl Product {
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

    pub fn get_startup_command(&self, output_dir: &PathBuf) -> Result<String, String> {
        let startup = match std::env::consts::OS {
            "windows" => &self.windows.startup.clone(),
            "macos" => &self.macos.startup.clone(),
            "linux" => &self.linux.startup.clone(),
            _ => return Err("Unsupported OS".to_string()),
        };

        let startup = template_replace_single(
            &startup,
            "output",
            &output_dir.to_string_lossy().into_owned(),
        );

        Ok(startup)
    }
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
