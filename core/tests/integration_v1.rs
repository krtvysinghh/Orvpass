use orvpass_core::version::constants::*;

#[test]
fn integration_release_identity() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(VERSION, "1.0.0");
}
