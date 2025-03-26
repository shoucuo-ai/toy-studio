use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
    process::Child,
    sync::{Arc, Mutex},
};

use tauri::AppHandle;

use crate::{
    get_file_name_without_suffix, git_clone, is_git_repository, split_args, uv_sync, uv_venv,
    Product,
};

use crate::AppConfig;

lazy_static! {
    pub static ref APP_INSTALLED: Mutex<HashMap<String, Option<Arc<Mutex<Child>>>>> =
        Mutex::new(HashMap::new());
}

/// 获取所有产品列表, 包括已安装和未安装的产品
#[tauri::command]
pub fn get_meta_product_list(app_handle: AppHandle) -> Result<String, String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let products = app_config.get_meta_product_list()?;
    serde_json::to_string(&products).map_err(|e| e.to_string())
}

/// 获取已安装的产品列表
#[tauri::command]
pub fn get_installed_product_list(app_handle: AppHandle) -> Result<String, String> {
    println!("--------------------------------get_installed_product_list--------------------------------");
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let all_products = app_config.get_meta_product_list()?;
    let installed_products: Vec<&Product> = all_products
        .iter()
        .filter(|product| APP_INSTALLED.lock().unwrap().contains_key(&product.id))
        .collect();
    serde_json::to_string(&installed_products).map_err(|e| e.to_string())
}

/// 安装产品
#[tauri::command]
pub fn product_install(app_handle: AppHandle, pid: String) -> Result<(), String> {
    println!("product_id:{}", pid);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_meta_products_dir();
    let product_file = product_dir.join(pid);
    let product = Product::parse_product_toml(&product_file)?;
    let product_name = get_file_name_without_suffix(&product.id);

    let install_dir = app_config.get_product_install_path().join(&product_name);
    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    println!("install_dir:{:?}", &install_dir);

    let git_url = product.download.git_url;
    let branch = product.download.branch;
    let bak_dir = app_config.get_product_bak_path();

    if !app_config.dev_mode() {
        // 1. git clone
        git_clone(git_url, branch, &install_dir, &bak_dir)?;

        // 2. create venv
        uv_venv(&install_dir, &product.download.python_version)?;

        // 3. sync
        uv_sync(&install_dir)?;
    } else {
        // git 目录存在

        let skip_clone = if install_dir.exists() {
            println!("install_dir exists:{}", install_dir.display());
            if is_git_repository(&install_dir) {
                println!("git repo: {}", install_dir.display());
                true
            } else {
                false
            }
        } else {
            false
        };

        let install_dir = install_dir.to_string_lossy().to_string();
        // 开发模式下使用cmd方式完成，合并为一个脚本
        let git_clone_cmd = format!("git clone -b {branch} {git_url} {install_dir}");
        let git_url_proxy = format!("https://ghfast.top/{}", git_url);
        let git_clone_cmd_proxy = format!("git clone -b {branch} {git_url_proxy} {install_dir}");
        let python_version = product.download.python_version;
        let cmd_script = if skip_clone {
            format!("cd {install_dir} && uv venv --python={python_version} && uv sync && pause")
        } else {
            format!(
                "({git_clone_cmd} || {git_clone_cmd_proxy}) && cd {install_dir} && uv venv --python={python_version} && uv sync && pause"
            )
        };
        let _ = crate::run_command(
            &product_dir,
            "",
            &vec![cmd_script],
            &product_name,
            &product.id,
        );
    }

    println!(
        "--------------------------------APP_INSTALLED insert:{}-----------------------",
        product.id
    );
    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

/// 重新安装产品
#[tauri::command]
pub fn product_reinstall(app_handle: AppHandle, pid: String) -> Result<(), String> {
    println!("product_id:{}", pid);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_meta_products_dir();
    let product_file = product_dir.join(pid);
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

    println!(
        "--------------------------------APP_INSTALLED insert:{}-----------------------",
        product.id
    );
    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

/// 卸载产品
#[tauri::command]
pub fn product_uninstall(app_handle: AppHandle, pid: String) -> Result<(), String> {
    println!("--------------------product_uninstall--------------------------------");
    println!("product_id:{}", pid);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_name = get_file_name_without_suffix(&pid);

    println!("product_name:{:?}", product_name);

    let install_dir = app_config.get_product_install_path().join(&product_name);
    println!("install_dir:{:?}", install_dir);

    fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;

    APP_INSTALLED.lock().unwrap().remove(&pid);

    Ok(())
}

/// 启动产品
#[tauri::command]
pub fn product_startup(app_handle: AppHandle, pid: String) -> Result<(), String> {
    println!("-----------------------product_startup--------------------------------");
    println!("product_id:{}", pid);

    // 1. get product info by product_id
    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_dir = app_config.get_meta_products_dir();
    let product_file = product_dir.join(&pid);
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

    // 3. get install_dir
    let install_dir = app_config.get_product_install_path().join(&product_name);
    println!("install_dir:{:?}", &install_dir);

    // 4. get output_dir & startup command
    let output_dir = app_config.get_output_path();
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    println!("output_dir:{:?}", &output_dir);

    let startup = product.get_startup_command(&output_dir)?;
    println!("startup:{}", startup);

    // 5. run startup command
    let mut args = split_args(&startup);
    args.insert(0, "run".to_string());
    println!("args:{:?}", args);


    let child = crate::run_command(install_dir, "uv", &args, &product_name, &product.id)?;

    APP_INSTALLED.lock().unwrap().insert(pid, Some(child));
    println!("APP_INSTALLED:{}", APP_INSTALLED.lock().unwrap().len());
    Ok(())
}

/// 关闭产品
#[tauri::command]
pub fn product_shutdown(pid: String) -> Result<(), String> {
    println!("product_shutdown:{}", pid);

    let mut child = APP_INSTALLED.lock().unwrap().remove(&pid);
    if let Some(Some(child)) = child.take() {
        child.lock().unwrap().kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 升级产品
#[tauri::command]
pub fn product_upgrade(app_handle: AppHandle, pid: String) -> Result<(), String> {
    println!("product_id:{}", pid);

    let app_config = AppConfig::get_app_config(&app_handle)?;

    let product_file = app_config.get_meta_product_dir(&pid);
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

    println!(
        "--------------------------------APP_INSTALLED insert:{}-----------------------",
        product.id
    );
    APP_INSTALLED.lock().unwrap().insert(product.id, None);

    Ok(())
}

/// 初始化已安装的产品
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
                println!("--------------------------------APP_INSTALLED insert:{}-----------------------", product_id);
                APP_INSTALLED.lock().unwrap().insert(product_id, None);
            }
        }
    }
    Ok(())
}

/// 初始化产品元数据
pub(crate) fn init_meta_products(app_handle: &AppHandle) -> Result<(), String> {
    let app_config = AppConfig::get_app_config(&app_handle)?;
    let meta_products_dir = app_config.get_meta_products_dir();

    let products_dir = Path::new("products");
    let canonicalize = products_dir.canonicalize().map_err(|e| e.to_string())?;
    println!("products_dir:{:?}", canonicalize);
    let product_files = fs::read_dir(&products_dir).map_err(|e| e.to_string())?;

    for product_file in product_files {
        match product_file {
            Err(err) => {
                println!("product_file list error:{}", err);
            }
            Ok(product_file) => {
                let product_file_path = product_file.path();
                let _ = fs::copy(product_file_path, &meta_products_dir);
            }
        }
    }
    Ok(())
}
