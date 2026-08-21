use orvpass_core::security::check;

#[test]
fn password_policy() {
    assert!(check::validate_master("StrongPassword123!"));
}

#[test]
fn entropy_works() {
    assert!(check::entropy_score("StrongPassword123!") > 0);
}
