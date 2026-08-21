use orvpass_core::build::*;

#[test]
fn production_build_identity() {
    assert_eq!(BUILD_CHANNEL, "stable");
    assert_eq!(BUILD_PROFILE, "release");
}
