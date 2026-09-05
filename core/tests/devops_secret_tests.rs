use orvpass_core::import_export::parsers::doppler::parse_doppler_json;

#[test]
fn test_doppler_json() {
    let json = "{\"DATABASE_URL\": \"postgres://localhost:5432\"}";
    let items = parse_doppler_json(json);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "DATABASE_URL");
}
