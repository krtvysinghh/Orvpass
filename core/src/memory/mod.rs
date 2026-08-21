use zeroize::Zeroize;

pub fn clear(value: &mut String) {
    value.zeroize();
}
