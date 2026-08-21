use std::panic;

pub fn install() {
    panic::set_hook(Box::new(|info| {
        eprintln!("Orvpass error: {}", info);
    }));
}

pub mod health;
