use orvpass_core::import_export::parsers::chrome::parse_chrome_csv;
use orvpass_core::import_export::parsers::apple::parse_apple_csv;

#[test]
fn test_chrome_csv_parsing() {
    let sample = "name,url,username,password,note\nGitHub,https://github.com,octocat,secret123,\n";
    let items = parse_chrome_csv(sample);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "GitHub");
}

#[test]
fn test_apple_csv_parsing() {
    let sample = "Title,URL,Username,Password,Notes,OTPAuth\nApple ID,https://apple.com,tim,steve123,,\n";
    let items = parse_apple_csv(sample);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "Apple ID");
}
