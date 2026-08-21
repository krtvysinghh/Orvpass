use orvpass_core::security::integrity;

#[test]
fn fingerprint_changes() {
    let a = integrity::fingerprint(b"one");
    let b = integrity::fingerprint(b"two");

    assert_ne!(a, b);
}
