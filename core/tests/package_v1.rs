use orvpass_core::version::constants::*;

#[test]
fn package_identity() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(CHANNEL, "stable");
}
