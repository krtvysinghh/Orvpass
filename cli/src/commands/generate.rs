use rand::Rng;

pub fn run() {
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";

    let mut rng = rand::rng();

    let password: String = (0..32)
        .map(|_| {
            let i = rng.random_range(0..chars.len());
            chars[i] as char
        })
        .collect();

    println!("{}", password);
}
