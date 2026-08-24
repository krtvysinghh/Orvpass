pub fn get_info() -> (&'static str, &'static str) {
    ("Orvpass", env!("CARGO_PKG_VERSION"))
}
