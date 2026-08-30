use zeroize::Zeroize;

pub fn clear(value: &mut String) {
    value.zeroize();
}

pub fn secure_memory_guard<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
