use orvpass_core::security::release;
use orvpass_core::version::check;

#[test]
fn release_ready() {
    assert!(release::verify_release());
    assert!(check::is_release_ready());
}
