pub fn secure_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;

    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }

    result == 0
}

pub fn sanitize(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}
