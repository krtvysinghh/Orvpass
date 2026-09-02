use orvpass_core::import_export::detect_vault_format;

#[test]
fn test_roundtrip_detection_stability() {
    let sample = "title,username,password,urls,notes";
    assert_eq!(detect_vault_format(sample), "CSV");
}
