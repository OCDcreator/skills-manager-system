use anyhow::Result;

use crate::core::platform_paths::normalize_portable_absolute_path;

pub fn normalize_project_path(project_path: &str) -> Result<String> {
    normalize_portable_absolute_path(project_path, "Project path")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn normalize_project_path_uses_portable_slashes_on_windows() {
        let normalized = normalize_project_path(r"  C:\Users\test\workspace\app\  ").unwrap();

        assert_eq!(normalized, "C:/Users/test/workspace/app");
    }

    #[cfg(windows)]
    #[test]
    fn normalize_project_path_coalesces_duplicate_windows_forms() {
        let backslash = normalize_project_path(r"C:\Users\test\workspace\app").unwrap();
        let slash = normalize_project_path("C:/Users/test/workspace/app/").unwrap();

        assert_eq!(backslash, slash);
    }
}
