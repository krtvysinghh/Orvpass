use crate::vault::database;
use orvpass_core::models::ItemData;

pub fn execute(json_output: bool, category_filter: Option<String>) {
    execute_filtered(json_output, category_filter, None)
}

pub fn execute_filtered(json_output: bool, category_filter: Option<String>, _tag: Option<String>) {
    let items = database::load_items();

    let filtered: Vec<_> = items
        .iter()
        .filter(|i| {
            if let Some(cat) = &category_filter {
                match cat.to_lowercase().as_str() {
                    "logins" | "login" => matches!(i.data, ItemData::Login(_)),
                    "notes" | "note" => matches!(i.data, ItemData::SecureNote(_)),
                    "cards" | "card" => matches!(i.data, ItemData::CreditCard(_)),
                    _ => true,
                }
            } else {
                true
            }
        })
        .collect();

    if json_output {
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap_or_default());
        return;
    }

    println!("📦 ORVPASS VAULT (Total: {} Items)", filtered.len());
    println!("{:<4} {:<24} {:<16} {:<28}", "TYPE", "TITLE", "CATEGORY", "DETAILS");
    println!("{}", "-".repeat(74));

    for item in filtered {
        let (icon, cat, detail) = match &item.data {
            ItemData::Login(l) => ("🔑", "Login", l.username.clone().unwrap_or_default()),
            ItemData::SecureNote(_) => ("📝", "Secure Note", "Confidential text".to_string()),
            ItemData::CreditCard(c) => ("💳", "Credit Card", format!("•••• {}", c.card_number.chars().rev().take(4).collect::<String>().chars().rev().collect::<String>())),
            _ => ("📦", "Custom", String::new()),
        };

        println!("{:<4} {:<24} {:<16} {:<28}", icon, item.title, cat, detail);
    }
}
