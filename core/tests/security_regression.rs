use orvpass_core::security;

#[test]
fn security_modules_load() {
    assert!(security::release::verify_release());
}
