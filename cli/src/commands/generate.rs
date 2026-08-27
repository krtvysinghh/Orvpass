use rand::Rng;

pub fn execute(length: usize, diceware: bool) -> String {
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
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";
        let mut rng = rand::rng();
        let len = length.max(8);
        (0..len)
            .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
            .collect()
    }
}
