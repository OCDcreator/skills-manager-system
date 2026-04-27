use anyhow::{anyhow, Result};
use std::path::Path;

pub fn normalize_project_path(project_path: &str) -> Result<String> {
    let project_path = project_path.trim();
    if project_path.is_empty() {
        return Err(anyhow!("Project path is required"));
    }

    let path = Path::new(project_path);
    if !path.is_absolute() {
        return Err(anyhow!("Project path must be absolute"));
    }

    Ok(project_path.to_string())
}
