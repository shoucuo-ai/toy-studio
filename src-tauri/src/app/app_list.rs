#[tauri::command]
pub fn get_app_list() -> Result<String, String> {
    let json = r#"[
    {
        "name": "app_name",
        "version": "1.0.0",
        "description": "app_description",
        "icon": "app_icon",
        "url": "https://github.com/Byaidu/PDFMathTranslate",
        "category": "app",
        "branch": "main",
        "status": "running",
        "created_at": "2021-01-01",
        "updated_at": "2021-01-01"
    }
    ]"#;

    Ok(json.to_string())
}
