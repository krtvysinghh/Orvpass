pub fn execute(length: usize) -> String {
    crate::password::generate(length, true, true)
}
