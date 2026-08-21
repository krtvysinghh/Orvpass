use orvpass_core::version::constants::*;

#[test]
fn quality_gate_identity() {
    assert_eq!(PRODUCT, "Orvpass");
    assert_eq!(VERSION, "1.0.0");
}
