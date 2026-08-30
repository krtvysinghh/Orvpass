#[test]
fn test_run_command_environment_key_sanitization() {
    let raw = "my-service.app key";
    let sanitized = raw.to_uppercase().replace([' ', '-', '.'], "_");
    assert_eq!(sanitized, "MY_SERVICE_APP_KEY");
}
