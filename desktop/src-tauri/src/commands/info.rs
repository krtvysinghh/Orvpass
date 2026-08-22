#[tauri::command]
pub fn get_app_info() -> String {
    "Orvpass v3.0 Rust Core".to_string()
}
