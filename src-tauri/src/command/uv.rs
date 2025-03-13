#[tauri::command]
pub fn uv_get_cache_dir() -> Result<String, String> {
    let output = std::process::Command::new("uv")
        .arg("cache")
        .arg("dir")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let cache_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(cache_dir)
}



#[tauri::command]
pub fn uv_get_python_envs() -> Result<String, String> {
    let output = std::process::Command::new("uv")
        .arg("python")
        .arg("list")
        .arg("--output-format")
        .arg("json")
        .arg("--python-preference")
        .arg("only-managed")
        .arg("--only-installed")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let envs = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    Ok(envs)
}
