use crate::vault::database;
use dialoguer::Input;
use orvpass_core::models::{
    CreditCardData, ItemData, ItemType, LoginData, SecureNoteData, VaultItem,
};

pub fn execute(
    title: Option<String>,
    username: Option<String>,
    password: Option<String>,
    note: Option<String>,
    category: Option<String>,
) -> anyhow::Result<()> {
    let mut items = database::load_items();

    let item_title = match title {
        Some(t) => t,
        None => Input::<String>::new()
            .with_prompt("Item Title / Service Name")
            .interact_text()?,
    };

    let cat = category.unwrap_or_else(|| "login".to_string());
    let new_item = match cat.to_lowercase().as_str() {
        "note" | "notes" => {
            let content = match note {
                Some(n) => n,
                None => Input::<String>::new()
                    .with_prompt("Secure Note Content")
                    .interact_text()?,
            };
            VaultItem::new(
                ItemType::SecureNote,
                &item_title,
                ItemData::SecureNote(SecureNoteData { content }),
            )
        }
        "card" | "cards" => {
            let cardholder = Input::<String>::new()
                .with_prompt("Cardholder Name")
                .interact_text()?;
            let number = Input::<String>::new()
                .with_prompt("Card Number")
                .interact_text()?;
            let exp_m = Input::<String>::new()
                .with_prompt("Expiry Month (MM)")
                .default("12".into())
                .interact_text()?;
            let exp_y = Input::<String>::new()
                .with_prompt("Expiry Year (YYYY)")
                .default("2028".into())
                .interact_text()?;
            let cvv = Input::<String>::new()
                .with_prompt("CVV")
                .default("123".into())
                .interact_text()?;
            VaultItem::new(
                ItemType::CreditCard,
                &item_title,
                ItemData::CreditCard(CreditCardData {
                    cardholder_name: cardholder,
                    card_number: number,
                    expiration_month: exp_m,
                    expiration_year: exp_y,
                    cvv,
                }),
            )
        }
        _ => {
            let user = match username {
                Some(u) => u,
                None => Input::<String>::new()
                    .with_prompt("Username / Email")
                    .interact_text()?,
            };
            let pass = match password {
                Some(p) => p,
                None => rpassword::prompt_password("Password (leave blank to generate): ")
                    .unwrap_or_default(),
            };
            let final_pass = if pass.is_empty() {
                crate::commands::generate::execute(20, false)
            } else {
                pass
            };

            VaultItem::new(
                ItemType::Login,
                &item_title,
                ItemData::Login(LoginData {
                    username: Some(user),
                    password: Some(final_pass),
                    urls: Vec::new(),
                }),
            )
        }
    };

    items.push(new_item);
    database::save_items(&items)?;
    println!("✨ Successfully saved '{}' to vault!", item_title);
    Ok(())
}
