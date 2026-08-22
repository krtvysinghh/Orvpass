#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::info::get_app_info;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_app_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running Orvpass");
}
