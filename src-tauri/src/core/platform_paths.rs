use anyhow::{anyhow, Result};
use std::path::Path;

pub fn normalize_portable_absolute_path(raw_path: &str, label: &str) -> Result<String> {
    let trimmed = raw_path.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{label} is required"));
    }

    let candidate = Path::new(trimmed);
    if !candidate.is_absolute() {
        return Err(anyhow!("{label} must be absolute"));
    }

    Ok(portable_path_string(candidate))
}

pub fn portable_path_string(path: &Path) -> String {
    let raw = path.to_string_lossy();
    let normalized = if cfg!(windows) {
        raw.replace('\\', "/")
    } else {
        raw.to_string()
    };
    trim_trailing_separators(&normalized)
}

fn trim_trailing_separators(path: &str) -> String {
    let mut normalized = path.to_string();
    while normalized.len() > root_prefix_len(&normalized) && normalized.ends_with('/') {
        normalized.pop();
    }
    normalized
}

fn root_prefix_len(path: &str) -> usize {
    if path == "/" {
        return 1;
    }

    let bytes = path.as_bytes();
    if bytes.len() >= 3 && bytes[1] == b':' && bytes[2] == b'/' {
        return 3;
    }

    if path.starts_with("//") {
        return 2;
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn portable_path_string_normalizes_backslashes_and_trims_tail() {
        let normalized = portable_path_string(Path::new(r"C:\Users\test\app\"));

        assert_eq!(normalized, "C:/Users/test/app");
    }

    #[cfg(not(windows))]
    #[test]
    fn portable_path_string_preserves_backslash_filename_characters() {
        let normalized = portable_path_string(Path::new("/Users/test/app\\name/"));

        assert_eq!(normalized, "/Users/test/app\\name");
    }

    #[test]
    fn portable_path_string_preserves_drive_root() {
        let normalized = portable_path_string(Path::new("C:/"));

        assert_eq!(normalized, "C:/");
    }
}
