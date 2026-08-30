use orvpass_core::models::ItemType;

#[test]
fn test_cli_item_type_serializes_correctly() {
    let item_type = ItemType::Login;
    let serialized = serde_json::to_string(&item_type).unwrap();
    assert_eq!(serialized, "\"Login\"");
}
