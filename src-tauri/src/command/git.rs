use std::path::Path;
use std::process::Command;

/// 执行git命令并返回结果
fn execute_git_command<P: AsRef<Path>>(dir: P, args: &[&str]) -> Result<String, String> {
    println!("execute_git_command:cd {} && git {:?}", dir.as_ref().to_string_lossy(), args);
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
fn is_git_repository<P: AsRef<Path>>(path: P) -> bool {
    Command::new("git")
        .current_dir(path)
        .args(&["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取git仓库的远程地址
fn get_remote_url<P: AsRef<Path>>(path: P) -> Result<String, String> {
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
fn update_repository<P: AsRef<Path>>(path: P) -> Result<String, String> {
    execute_git_command(path, &["pull"])
}

/// 克隆git仓库
fn clone_repository<P: AsRef<Path>>(url: &str, branch: String, path: P) -> Result<String, String> {
    println!("clone_repository:{}", path.as_ref().to_string_lossy());
    execute_git_command(
        &Path::new("."),
        &[
            "clone",
            url,
            "-b",
            &branch,
            "--single-branch",
            &path.as_ref().to_string_lossy(),
        ],
    )
}

/// 验证远程地址是否匹配
fn verify_remote_url<P: AsRef<Path>>(path: P, expected_url: &str) -> Result<(), String> {
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

pub fn git_clone<P: AsRef<Path>>(
    url: String,
    branch: String,
    path: P,
    bak: P,
) -> Result<String, String> {
    let path = path.as_ref();
    let bak = bak.as_ref();
    if path.exists() {
        println!("path exists:{}", path.display());
        if !is_git_repository(&path) {
            let _ = move_to_bak(&path, &bak);
            clone_repository(&url, branch, path)
        } else {
            verify_remote_url(&path, &url)?;
            update_repository(&path)
        }
    } else {
        clone_repository(&url, branch, &path)
    }
}
fn move_to_bak<P: AsRef<Path>>(from: P, to: P) -> Result<(), String> {
    println!(
        "move_to_bak:{} to {}",
        from.as_ref().to_string_lossy(),
        to.as_ref().to_string_lossy()
    );
    std::fs::rename(from, to).map_err(|e| e.to_string())
}
