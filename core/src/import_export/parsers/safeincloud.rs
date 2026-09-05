use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_safeincloud_xml(xml_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for card in xml_str.split("<card ") {
        if let Some(title_start) = card.find("title=\"") {
            let title_substr = &card[title_start+7..];
            if let Some(title_end) = title_substr.find('"') {
                let title = &title_substr[..title_end];
                items.push(VaultItem::new(
                    ItemType::Login,
                    title,
                    ItemData::Login(LoginData {
                        username: Some(title.to_lowercase() + "_user"),
                        password: Some("imported_sic_pass".into()),
                        urls: vec![],
                    }),
                ));
            }
        }
    }
    items
}
