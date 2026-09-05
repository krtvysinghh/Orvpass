use orvpass_core::import_export::parsers::enpass::parse_enpass_json;

#[test]
fn test_enpass_parser_empty() {
    let items = parse_enpass_json("{}");
    assert!(items.is_empty());
}
