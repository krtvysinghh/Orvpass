pub type Result<T> = std::result::Result<T, String>;

pub fn ok<T>(value: T) -> Result<T> {
    Ok(value)
}
