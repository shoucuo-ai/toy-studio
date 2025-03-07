#[tauri::command]
fn get_python_envs() -> Result<Vec<String>, String> {
    let output = std::process::Command::new("uv")
        .arg("envs")
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