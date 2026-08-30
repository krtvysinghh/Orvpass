use rand::Rng;

pub fn execute(length: usize, diceware: bool) -> String {
    execute_advanced(length, diceware, false, true)
}

pub fn execute_advanced(length: usize, diceware: bool, avoid_ambiguous: bool, include_symbols: bool) -> String {
    if diceware {
        let words = [
            "correct", "horse", "battery", "staple", "crypto", "shield", "beacon", "galaxy",
            "orbit", "quantum", "falcon", "liquid", "zenith", "timber", "solace", "nexus",
            "prism", "shadow", "cyber", "vector", "radiant", "glacier", "ember", "titan",
        ];
        let mut rng = rand::rng();
        let chosen: Vec<&str> = (0..4).map(|_| words[rng.random_range(0..words.len())]).collect();
        let num: u32 = rng.random_range(10..99);
        format!("{}-{}", chosen.join("-"), num)
    } else {
        let mut charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string();
        if include_symbols {
            charset.push_str("!@#$%^&*()_+-=");
        }
        if avoid_ambiguous {
            charset = charset.chars().filter(|c| !matches!(c, 'l' | '1' | 'I' | 'O' | '0')).collect();
        }
        let bytes = charset.as_bytes();
        let mut rng = rand::rng();
        let len = length.max(8);
        (0..len)
            .map(|_| bytes[rng.random_range(0..bytes.len())] as char)
            .collect()
    }
}
