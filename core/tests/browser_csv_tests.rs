use orvpass_core::import_export::parsers::firefox::parse_firefox_csv;
use orvpass_core::import_export::parsers::edge::parse_edge_csv;

#[test]
fn test_firefox_csv() {
    let csv = "\"url\",\"username\",\"password\",\"realm\"\n\"https://site.com\",\"admin\",\"secret\",\"\"\n";
    let items = parse_firefox_csv(csv);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "https://site.com");
}

#[test]
fn test_edge_csv() {
    let csv = "name,url,username,password,note\nEdgeSite,https://edge.com,user,pass,\n";
    let items = parse_edge_csv(csv);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "EdgeSite");
}
