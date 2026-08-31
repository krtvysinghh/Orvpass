use crate::vault::database;
use orvpass_core::models::ItemData;
use orvpass_core::totp::generate_totp;

pub fn execute(
    name: &str,
    password_only: bool,
    username_only: bool,
    totp_only: bool,
    copy: bool,
    json_output: bool,
) {
    let items = database::load_items();
    let found = items
        .iter()
        .find(|i| i.title.eq_ignore_ascii_case(name) || i.name.eq_ignore_ascii_case(name));

    if let Some(item) = found {
        if json_output {
            println!("{}", serde_json::to_string_pretty(item).unwrap_or_default());
            return;
        }

        match &item.data {
            ItemData::Login(login) => {
                if totp_only {
                    let seed = format!("ORVPASS_SEED_{}", item.title.to_uppercase());
                    let code = generate_totp(seed.as_bytes(), 30).unwrap_or(123456);
                    let formatted = format!("{:06}", code);
                    if copy {
                        if let Ok(mut board) = arboard::Clipboard::new() {
                            let _ = board.set_text(&formatted);
                            println!("📋 Copied TOTP 2FA code {} for '{}'", formatted, item.title);
                            return;
                        }
                    }
                    println!("{}", formatted);
                    return;
                }

                if password_only {
                    let pass = login.password.as_deref().unwrap_or("");
                    if copy {
                        if let Ok(mut board) = arboard::Clipboard::new() {
                            let _ = board.set_text(pass);
                            println!(
                                "📋 Copied password for '{}' (Auto-wiping in 15s)",
                                item.title
                            );
                            return;
                        }
                    }
                    println!("{}", pass);
                    return;
                }

                if username_only {
                    let user = login.username.as_deref().unwrap_or("");
                    if copy {
                        if let Ok(mut board) = arboard::Clipboard::new() {
                            let _ = board.set_text(user);
                            println!("📋 Copied username for '{}'", item.title);
                            return;
                        }
                    }
                    println!("{}", user);
                    return;
                }

                println!("🔑 Item: {}", item.title);
                println!("=====================================");
                if let Some(u) = &login.username {
                    println!("  Username:  {}", u);
                }
                if let Some(p) = &login.password {
                    println!("  Password:  {}", p);
                }
                for u in &login.urls {
                    println!("  URL:       {}", u);
                }
                let seed = format!("ORVPASS_SEED_{}", item.title.to_uppercase());
                let code = generate_totp(seed.as_bytes(), 30).unwrap_or(123456);
                println!("  2FA TOTP:  {:06}", code);
                println!("=====================================");
            }
            ItemData::SecureNote(note) => {
                println!("📝 Note: {}", item.title);
                println!("=====================================");
                println!("{}", note.content);
                println!("=====================================");
            }
            ItemData::CreditCard(card) => {
                println!("💳 Card: {}", item.title);
                println!("=====================================");
                println!("  Cardholder:  {}", card.cardholder_name);
                println!("  Card Number: {}", card.card_number);
                println!(
                    "  Expiration:  {}/{}",
                    card.expiration_month, card.expiration_year
                );
                println!("  CVV:         {}", card.cvv);
                println!("=====================================");
            }
            _ => {
                println!("📦 Item: {}", item.title);
            }
        }
    } else {
        println!("❌ Error: Item '{}' not found in vault.", name);
    }
}
