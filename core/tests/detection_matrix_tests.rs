use orvpass_core::import_export::detect_vault_format;

#[test]
fn test_all_format_signatures() {
    assert_eq!(detect_vault_format("{\"items\": []}"), "Bitwarden");
    assert_eq!(detect_vault_format("<KeePassFile>"), "KeePass");
    assert_eq!(detect_vault_format("name,url,username,password"), "Chrome");
}
