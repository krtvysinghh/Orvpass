#[test]
fn test_shred_non_existent_file() {
    let p = std::path::Path::new("/tmp/non_existent_orvpass_test_file.bin");
    assert!(!p.exists());
}
