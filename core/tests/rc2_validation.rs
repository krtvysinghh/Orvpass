use orvpass_core::version::constants::*;

#[test]
fn rc2_validation_identity() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(VERSION, "1.0.0");
}
