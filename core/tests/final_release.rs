use orvpass_core::security;

#[test]
fn final_security_gate() {
    assert!(security::final_check::final_security_check());
}
