use rand::Rng;

pub fn generate(length: usize) -> String {

let chars =
b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";

let mut rng = rand::thread_rng();

(0..length)
.map(|_| chars[rng.gen_range(0..chars.len())] as char)
.collect()

}
