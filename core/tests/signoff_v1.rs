use orvpass_core::version::constants::*;

#[test]
fn v1_signoff() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(VERSION, "1.0.0");
}
