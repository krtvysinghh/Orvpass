use orvpass_core::crypto::SecretKey;
use orvpass_core::models::{
    CreditCardData, ItemData, ItemType, LoginData, SecureNoteData, VaultItem,
};
use orvpass_core::vault::Vault;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    vault_data: Mutex<Option<(Vault, SecretKey)>>,
}

#[derive(Serialize)]
struct VaultStatus {
    exists: bool,
    unlocked: bool,
}

fn resolve_vault_path(app: &tauri::AppHandle) -> PathBuf {
    let base_dir = app
        .path()
        .app_data_dir()
        .or_else(|_| dirs::data_dir().ok_or(()))
        .or_else(|_| dirs::home_dir().ok_or(()))
        .unwrap_or_else(|_| PathBuf::from("."));

    base_dir.join("orvpass_vault.enc")
}

#[tauri::command]
fn check_vault_status(
    app: tauri::AppHandle,
    state: State<AppState>,
) -> Result<VaultStatus, String> {
    let path = resolve_vault_path(&app);
    let guard = state.vault_data.lock().unwrap();
    Ok(VaultStatus {
        exists: path.exists(),
        unlocked: guard.is_some(),
    })
}

#[tauri::command]
fn unlock_vault(
    app: tauri::AppHandle,
    state: State<AppState>,
    password: &str,
) -> Result<bool, String> {
    let path = resolve_vault_path(&app);
    if !path.exists() {
        return Err("Vault does not exist. Please create a new vault.".into());
    }

    let mut v = Vault::new_locked_at(&path);
    let key = SecretKey::from_password(password).map_err(|e| e.to_string())?;

    v.unlock(&key).map_err(|e| match e {
        orvpass_core::vault::VaultError::TemporarilyLocked => {
            "Vault is temporarily locked due to too many failed attempts.".to_string()
        }
        _ => "Incorrect master password. Please try again.".to_string(),
    })?;

    *state.vault_data.lock().unwrap() = Some((v, key));
    Ok(true)
}

#[tauri::command]
fn create_vault(
    app: tauri::AppHandle,
    state: State<AppState>,
    password: &str,
) -> Result<bool, String> {
    let path = resolve_vault_path(&app);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut v = Vault::new_locked_at(&path);
    let key = SecretKey::from_password(password).map_err(|e| e.to_string())?;

    v.initialize(&key).map_err(|e| e.to_string())?;
    *state.vault_data.lock().unwrap() = Some((v, key));
    Ok(true)
}

#[tauri::command]
fn lock_vault(state: State<AppState>) -> Result<bool, String> {
    let mut guard = state.vault_data.lock().unwrap();
    *guard = None;
    Ok(true)
}

#[tauri::command]
fn initialize_vault(
    app: tauri::AppHandle,
    state: State<AppState>,
    password: &str,
) -> Result<String, String> {
    let path = resolve_vault_path(&app);
    let mut v = Vault::new_locked_at(&path);
    let key = SecretKey::from_password(password).map_err(|e| e.to_string())?;

    if path.exists() {
        v.unlock(&key)
            .map_err(|_| "Incorrect master password".to_string())?;
        *state.vault_data.lock().unwrap() = Some((v, key));
        Ok("Unlocked".into())
    } else {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        v.initialize(&key).map_err(|e| e.to_string())?;
        *state.vault_data.lock().unwrap() = Some((v, key));
        Ok("Initialized".into())
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
    exp_month: Option<String>,
    exp_year: Option<String>,
) -> Result<(), String> {
    let mut guard = state.vault_data.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        let itype = item_type.as_deref().unwrap_or("Logins");
        let item = match itype {
            "Secure Notes" => {
                let note = SecureNoteData {
                    content: notes.unwrap_or_default(),
                };
                VaultItem::new(ItemType::SecureNote, &title, ItemData::SecureNote(note))
            }
            "Credit Cards" => {
                let card = CreditCardData {
                    cardholder_name: username.unwrap_or_default(),
                    card_number: cc.unwrap_or_default(),
                    expiration_month: exp_month.unwrap_or_else(|| "12".into()),
                    expiration_year: exp_year.unwrap_or_else(|| "28".into()),
                    cvv: pass.unwrap_or_default(),
                };
                VaultItem::new(ItemType::CreditCard, &title, ItemData::CreditCard(card))
            }
            _ => {
                let login = LoginData {
                    username,
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
        Err("Vault is locked".into())
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
        Err("Vault is locked".into())
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
            check_vault_status,
            unlock_vault,
            create_vault,
            lock_vault,
            initialize_vault,
            get_items,
            add_item,
            delete_item,
            generate_password
        ]);

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_global_shortcut::Builder::new().build());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
