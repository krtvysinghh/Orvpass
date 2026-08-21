#[derive(Debug)]
pub enum Level {
    Info,
    Warning,
    Error,
}

pub fn write(level: Level, msg: &str) {
    println!("[{:?}] {}", level, msg);
}
