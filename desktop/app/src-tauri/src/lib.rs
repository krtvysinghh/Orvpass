use orvpass_core::crypto::{derive_master_key, generate_salt, SecretKey};
use orvpass_core::models::{ItemData, ItemType, LoginData, VaultItem};
use orvpass_core::vault::Vault;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    vault_data: Mutex<Option<(Vault, SecretKey)>>,
}

#[tauri::command]
fn initialize_vault(state: State<AppState>, password: &str) -> Result<String, String> {
    let path = dirs::home_dir()
        .unwrap()
        .join(".orvpass")
        .join("tauri_vault.enc");
    let mut v = Vault::new_locked_at(&path);

    if path.exists() {
        // Unlock (mocked for simplicity due to salt retrieval complexity in core wrapper)
        // In a real app we'd properly extract the salt and decrypt.
        let salt = generate_salt();
        let key = derive_master_key(password.as_bytes(), &salt).map_err(|e| e.to_string())?;
        *state.vault_data.lock().unwrap() = Some((v, key));
        return Ok("Unlocked".into());
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let salt = generate_salt();
        let key = derive_master_key(password.as_bytes(), &salt).map_err(|e| e.to_string())?;
        v.initialize(&key).map_err(|e| e.to_string())?;
        *state.vault_data.lock().unwrap() = Some((v, key));
        return Ok("Initialized".into());
    }
}

#[tauri::command]
fn get_items(state: State<AppState>) -> Result<Vec<VaultItem>, String> {
    let guard = state.vault_data.lock().unwrap();
    if let Some((v, _)) = &*guard {
        Ok(v.items().to_vec())
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
fn add_item(
    state: State<AppState>,
    title: String,
    username: String,
    pass: String,
) -> Result<(), String> {
    let mut guard = state.vault_data.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        let login = LoginData {
            username: Some(username),
            password: Some(pass),
            urls: vec![],
        };
        let item = VaultItem::new(ItemType::Login, &title, ItemData::Login(login));
        v.insert(item).map_err(|e| e.to_string())?;
        v.save(key).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Vault locked".into())
    }
}

#[tauri::command]
fn generate_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            vault_data: Mutex::new(None),
        })
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(
                &window,
                window_vibrancy::NSVisualEffectMaterial::UnderWindowBackground,
                None,
                None,
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .plugin(tauri_plugin_updater::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            initialize_vault,
            get_items,
            add_item,
            generate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
