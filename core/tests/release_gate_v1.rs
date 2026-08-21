use orvpass_core::version::constants::*;

#[test]
fn release_gate_passes() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(VERSION, "1.0.0");
}
