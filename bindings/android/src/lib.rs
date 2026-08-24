use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jstring};
use jni::JNIEnv;
use orvpass_core::crypto::SecretKey;
use orvpass_core::models::{CreditCardData, ItemData, ItemType, LoginData, SecureNoteData, VaultItem};
use orvpass_core::vault::Vault;
use std::path::PathBuf;
use std::sync::Mutex;

static VAULT_STATE: Mutex<Option<(Vault, SecretKey)>> = Mutex::new(None);

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_checkVaultStatus(
    mut env: JNIEnv,
    _class: JClass,
    path_str: JString,
) -> jstring {
    let path: String = match env.get_string(&path_str) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("{\"exists\":false,\"unlocked\":false}").unwrap().into_raw(),
    };

    let p = PathBuf::from(path);
    let guard = VAULT_STATE.lock().unwrap();
    let status_json = format!(
        "{{\"exists\":{},\"unlocked\":{}}}",
        p.exists(),
        guard.is_some()
    );

    env.new_string(status_json).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_createVault(
    mut env: JNIEnv,
    _class: JClass,
    path_str: JString,
    password_str: JString,
) -> jboolean {
    let path: String = match env.get_string(&path_str) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };
    let password: String = match env.get_string(&password_str) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let p = PathBuf::from(path);
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut v = Vault::new_locked_at(&p);
    let key = match SecretKey::from_password(&password) {
        Ok(k) => k,
        Err(_) => return 0,
    };

    if v.initialize(&key).is_ok() {
        *VAULT_STATE.lock().unwrap() = Some((v, key));
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_unlockVault(
    mut env: JNIEnv,
    _class: JClass,
    path_str: JString,
    password_str: JString,
) -> jboolean {
    let path: String = match env.get_string(&path_str) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };
    let password: String = match env.get_string(&password_str) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let p = PathBuf::from(path);
    if !p.exists() {
        return 0;
    }

    let mut v = Vault::new_locked_at(&p);
    let key = match SecretKey::from_password(&password) {
        Ok(k) => k,
        Err(_) => return 0,
    };

    if v.unlock(&key).is_ok() {
        *VAULT_STATE.lock().unwrap() = Some((v, key));
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_lockVault(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    let mut guard = VAULT_STATE.lock().unwrap();
    *guard = None;
    1
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_getItemsJson(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let guard = VAULT_STATE.lock().unwrap();
    let json = if let Some((v, _)) = &*guard {
        serde_json::to_string(v.items()).unwrap_or_else(|_| "[]".into())
    } else {
        "[]".into()
    };

    env.new_string(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_addItem(
    mut env: JNIEnv,
    _class: JClass,
    type_str: JString,
    title_str: JString,
    user_str: JString,
    pass_str: JString,
    notes_str: JString,
    cc_str: JString,
    exp_month_str: JString,
    exp_year_str: JString,
) -> jboolean {
    let item_type: String = env.get_string(&type_str).map(|s| s.into()).unwrap_or_default();
    let title: String = env.get_string(&title_str).map(|s| s.into()).unwrap_or_default();
    let username: Option<String> = env.get_string(&user_str).ok().map(|s| s.into()).filter(|s: &String| !s.is_empty());
    let password: Option<String> = env.get_string(&pass_str).ok().map(|s| s.into()).filter(|s: &String| !s.is_empty());
    let notes: Option<String> = env.get_string(&notes_str).ok().map(|s| s.into()).filter(|s: &String| !s.is_empty());
    let cc: Option<String> = env.get_string(&cc_str).ok().map(|s| s.into()).filter(|s: &String| !s.is_empty());
    let exp_month: String = env.get_string(&exp_month_str).map(|s| s.into()).unwrap_or_else(|_| "12".into());
    let exp_year: String = env.get_string(&exp_year_str).map(|s| s.into()).unwrap_or_else(|_| "28".into());

    let mut guard = VAULT_STATE.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        let item = match item_type.as_str() {
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
                    expiration_month: exp_month,
                    expiration_year: exp_year,
                    cvv: password.unwrap_or_default(),
                };
                VaultItem::new(ItemType::CreditCard, &title, ItemData::CreditCard(card))
            }
            _ => {
                let login = LoginData {
                    username,
                    password,
                    urls: vec![],
                };
                VaultItem::new(ItemType::Login, &title, ItemData::Login(login))
            }
        };

        if v.insert(item).is_ok() && v.save(key).is_ok() {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_deleteItem(
    mut env: JNIEnv,
    _class: JClass,
    id_str: JString,
) -> jboolean {
    let id: String = match env.get_string(&id_str) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let mut guard = VAULT_STATE.lock().unwrap();
    if let Some((v, key)) = &mut *guard {
        if v.delete(id).is_ok() && v.save(key).is_ok() {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_krtvysingh_orvpass_bridge_OrvpassNativeBridge_generatePassword(
    env: JNIEnv,
    _class: JClass,
    length: jint,
) -> jstring {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::rng();
    let len = if length > 0 { length as usize } else { 16 };
    let pass: String = (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    env.new_string(pass).unwrap().into_raw()
}
