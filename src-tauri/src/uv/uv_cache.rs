#[tauri::command]
pub fn get_uv_cache_dir() -> Result<String, String> {
  let cache_dir = uv_cache::cache_dir()
      .map_err(|e| e.to_string())?
      .to_string_lossy()
      .into_owned();
  Ok(cache_dir)
}
