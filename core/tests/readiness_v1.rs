use orvpass_core::security::readiness;

#[test]
fn production_readiness() {
    assert!(readiness::production_ready());
}
