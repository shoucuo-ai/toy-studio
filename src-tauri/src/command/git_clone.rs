use std::path::Path;
use std::process::Command;

/// 执行git命令并返回结果
fn execute_git_command(dir: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// 检查目录是否是git仓库
fn is_git_repository(path: &Path) -> bool {
    Command::new("git")
        .current_dir(path)
        .args(&["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取git仓库的远程地址
fn get_remote_url(path: &Path) -> Result<String, String> {
    execute_git_command(path, &["config", "--get", "remote.origin.url"])
}

/// 标准化git仓库URL
fn normalize_url(url: &str) -> String {
    url.trim()
        .trim_end_matches(".git")
        .trim_end_matches('/')
        .to_string()
}

/// 更新已存在的git仓库
fn update_repository(path: &Path) -> Result<String, String> {
    execute_git_command(path, &["pull", "--ff-only"])
}

/// 克隆git仓库
fn clone_repository(url: &str, path: &Path) -> Result<String, String> {
    execute_git_command(Path::new("."), &["clone", url, &path.to_string_lossy()])
}

/// 验证远程地址是否匹配
fn verify_remote_url(path: &Path, expected_url: &str) -> Result<(), String> {
    let current_url = get_remote_url(path)?;
    let current_url = normalize_url(&current_url);
    let expected_url = normalize_url(expected_url);

    if current_url != expected_url {
        return Err(format!(
            "目标目录已存在git仓库，但远程地址不匹配。\n现有远程地址: {}\n请求克隆地址: {}",
            current_url, expected_url
        ));
    }
    Ok(())
}

#[tauri::command]
pub fn git_clone(url: String, path: String) -> Result<String, String> {
    let path = Path::new(&path);

    if path.exists() {
        if !is_git_repository(path) {
            return Err(format!("目标路径 '{}' 已存在且不是git仓库", path.display()));
        }

        verify_remote_url(path, &url)?;
        update_repository(path)
    } else {
        clone_repository(&url, path)
    }
}
