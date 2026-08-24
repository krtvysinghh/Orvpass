use orvpass_core::crypto::SecretKey;
use orvpass_core::models::{ItemData, ItemType, LoginData, VaultItem};
use orvpass_core::vault::Vault;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    vault_data: Mutex<Option<(Vault, SecretKey)>>,
}

#[tauri::command]
fn initialize_vault(
    app: tauri::AppHandle,
    state: State<AppState>,
    password: &str,
) -> Result<String, String> {
    let base_dir = app
        .path()
        .app_data_dir()
        .or_else(|_| dirs::data_dir().ok_or(()))
        .or_else(|_| dirs::home_dir().ok_or(()))
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    let path = base_dir.join("orvpass_vault.enc");
    let mut v = Vault::new_locked_at(&path);

    let key = SecretKey::from_password(password).map_err(|e| e.to_string())?;

    if path.exists() {
        if let Err(_) = v.unlock(&key) {
            let _ = v.initialize(&key);
        }
        *state.vault_data.lock().unwrap() = Some((v, key));
        return Ok("Unlocked".into());
    } else {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
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
    item_type: Option<String>,
    title: String,
    username: Option<String>,
    pass: Option<String>,
    notes: Option<String>,
    cc: Option<String>,
) -> Result<(), String> {
    let mut guard = state.vault_data.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        let itype = item_type.as_deref().unwrap_or("Logins");
        let item = match itype {
            "Secure Notes" => {
                let note = orvpass_core::models::SecureNoteData {
                    content: notes.unwrap_or_default(),
                };
                VaultItem::new(ItemType::SecureNote, &title, ItemData::SecureNote(note))
            }
            "Credit Cards" => {
                let card = orvpass_core::models::CreditCardData {
                    cardholder_name: username.unwrap_or_default(),
                    card_number: cc.unwrap_or_default(),
                    expiration_month: "12".into(),
                    expiration_year: "28".into(),
                    cvv: pass.unwrap_or_default(),
                };
                VaultItem::new(ItemType::CreditCard, &title, ItemData::CreditCard(card))
            }
            _ => {
                let login = LoginData {
                    username: username,
                    password: pass,
                    urls: vec![],
                };
                VaultItem::new(ItemType::Login, &title, ItemData::Login(login))
            }
        };
        v.insert(item).map_err(|e| e.to_string())?;
        v.save(key).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Vault locked".into())
    }
}

#[tauri::command]
fn delete_item(state: State<AppState>, id: String) -> Result<(), String> {
    let mut guard = state.vault_data.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        v.delete(id).map_err(|e| e.to_string())?;
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
    let mut builder = tauri::Builder::default()
        .manage(AppState {
            vault_data: Mutex::new(None),
        })
        .setup(|_app| {
            #[cfg(desktop)]
            if let Some(window) = _app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                let _ = window_vibrancy::apply_vibrancy(
                    &window,
                    window_vibrancy::NSVisualEffectMaterial::UnderWindowBackground,
                    None,
                    None,
                );

                #[cfg(target_os = "windows")]
                let _ = window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)));
            }

            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            initialize_vault,
            get_items,
            add_item,
            delete_item,
            generate_password
        ]);

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_global_shortcut::Builder::new().build());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
