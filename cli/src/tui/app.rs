use orvpass_core::models::{VaultItem, ItemData};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    All,
    Logins,
    Notes,
    Cards,
    Favorites,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::All,
            Category::Logins,
            Category::Notes,
            Category::Cards,
            Category::Favorites,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Category::All => "All Items",
            Category::Logins => "Logins",
            Category::Notes => "Secure Notes",
            Category::Cards => "Credit Cards",
            Category::Favorites => "Favorites",
        }
    }
}

pub struct TuiApp {
    pub items: Vec<VaultItem>,
    pub selected_index: usize,
    pub category: Category,
    pub search_query: String,
    pub is_searching: bool,
    pub show_password: bool,
    pub status_message: Option<(String, SystemTime)>,
    pub should_quit: bool,
}

impl TuiApp {
    pub fn new(items: Vec<VaultItem>) -> Self {
        Self {
            items,
            selected_index: 0,
            category: Category::All,
            search_query: String::new(),
            is_searching: false,
            show_password: false,
            status_message: None,
            should_quit: false,
        }
    }

    pub fn filtered_items(&self) -> Vec<&VaultItem> {
        self.items
            .iter()
            .filter(|item| {
                let matches_cat = match self.category {
                    Category::All => true,
                    Category::Logins => matches!(item.data, ItemData::Login(_)),
                    Category::Notes => matches!(item.data, ItemData::SecureNote(_)),
                    Category::Cards => matches!(item.data, ItemData::CreditCard(_)),
                    Category::Favorites => item.tags.iter().any(|t| t == "favorite" || t == "pinned"),
                };

                let query = self.search_query.to_lowercase();
                let matches_search = query.is_empty()
                    || item.title.to_lowercase().contains(&query)
                    || item.name.to_lowercase().contains(&query);

                matches_cat && matches_search
            })
            .collect()
    }

    pub fn selected_item(&self) -> Option<&VaultItem> {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            None
        } else {
            let idx = self.selected_index.min(filtered.len().saturating_sub(1));
            Some(filtered[idx])
        }
    }

    pub fn next(&mut self) {
        let count = self.filtered_items().len();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    pub fn previous(&mut self) {
        let count = self.filtered_items().len();
        if count > 0 {
            if self.selected_index == 0 {
                self.selected_index = count - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn next_category(&mut self) {
        let categories = Category::all();
        let current_idx = categories.iter().position(|c| *c == self.category).unwrap_or(0);
        self.category = categories[(current_idx + 1) % categories.len()].clone();
        self.selected_index = 0;
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some((msg.to_string(), SystemTime::now()));
    }

    pub fn totp_seconds_left() -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        30 - (now % 30)
    }
}
