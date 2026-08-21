use std::fs;

pub fn secure_file(path: &std::path::Path) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mode = meta.permissions().mode();

            return mode & 0o077 == 0;
        }

        #[cfg(not(unix))]
        {
            return true;
        }
    }

    false
}
