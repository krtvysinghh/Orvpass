use std::path::Path;

pub fn validate(path: &Path) -> bool {
    path.exists()
}
