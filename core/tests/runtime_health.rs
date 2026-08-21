use orvpass_core::runtime::health::HealthReport;

#[test]
fn health_defaults() {
    let h = HealthReport::ok();

    assert!(h.encrypted);
    assert!(h.locked);
}
