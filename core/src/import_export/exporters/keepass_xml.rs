use crate::models::{ItemData, VaultItem};

pub fn export_keepass_xml(items: &[VaultItem]) -> String {
    let mut entries = String::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            entries.push_str(&format!(
                r#"    <Entry>
      <String><Key>Title</Key><Value>{}</Value></String>
      <String><Key>UserName</Key><Value>{}</Value></String>
      <String><Key>Password</Key><Value>{}</Value></String>
    </Entry>
"#,
                item.title,
                l.username.as_deref().unwrap_or(""),
                l.password.as_deref().unwrap_or("")
            ));
        }
    }
    format!(
        r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<KeePassFile>
  <Root>
    <Group>
      <Name>Orvpass Vault</Name>
{}    </Group>
  </Root>
</KeePassFile>"#,
        entries
    )
}
