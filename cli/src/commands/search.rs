use crate::vault::database;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use orvpass_core::models::ItemData;

pub fn execute(query: &str) {
    search_with_tag(query, None)
}

pub fn search_with_tag(query: &str, _tag: Option<&str>) {
    let items = database::load_items();
    let matcher = SkimMatcherV2::default();

    let mut matches: Vec<(&orvpass_core::models::VaultItem, i64)> = items
        .iter()
        .filter_map(|item| {
            let score1 = matcher.fuzzy_match(&item.title, query).unwrap_or(0);
            let score2 = matcher.fuzzy_match(&item.name, query).unwrap_or(0);
            let score = score1.max(score2);
            if score > 0 { Some((item, score)) } else { None }
        })
        .collect();

    matches.sort_by(|a, b| b.1.cmp(&a.1));

    if matches.is_empty() {
        println!("🔍 No matching credentials found for '{}'.", query);
        return;
    }

    println!(
        "🔍 Search results for '{}' ({} matches):",
        query,
        matches.len()
    );
    println!("{:<4} {:<24} {:<28}", "TYPE", "TITLE", "DETAILS");
    println!("{}", "-".repeat(60));

    for (item, _) in matches {
        let (icon, detail) = match &item.data {
            ItemData::Login(l) => ("🔑", l.username.clone().unwrap_or_default()),
            ItemData::SecureNote(_) => ("📝", "Secure Note".to_string()),
            ItemData::CreditCard(c) => (
                "💳",
                format!(
                    "•••• {}",
                    c.card_number
                        .chars()
                        .rev()
                        .take(4)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect::<String>()
                ),
            ),
            _ => ("📦", String::new()),
        };
        println!("{:<4} {:<24} {:<28}", icon, item.title, detail);
    }
}
