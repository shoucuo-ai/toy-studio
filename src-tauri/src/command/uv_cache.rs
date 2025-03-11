#[tauri::command]
pub fn get_uv_cache_dir() -> Result<String, String> {
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
