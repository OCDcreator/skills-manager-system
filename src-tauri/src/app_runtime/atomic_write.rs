use anyhow::{Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(windows))]
use std::fs::File;

pub fn write_text_atomic(path: &Path, content: &str) -> Result<()> {
    let parent = path
        .parent()
        .context("Atomic write requires a parent directory")?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {:?}", parent))?;

    let temp_path = temporary_path(path);
    let write_result = write_temp_file(&temp_path, content);
    if let Err(error) = write_result {
        let _ = fs::remove_file(&temp_path);
        return Err(error);
    }

    let swap_result = replace_path(&temp_path, path);
    if let Err(error) = swap_result {
        let _ = fs::remove_file(&temp_path);
        return Err(error);
    }

    sync_directory(parent)?;
    Ok(())
}

fn write_temp_file(path: &Path, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("Failed to create atomic temp file {:?}", path))?;
    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write atomic temp file {:?}", path))?;
    file.sync_all()
        .with_context(|| format!("Failed to sync atomic temp file {:?}", path))?;
    Ok(())
}

fn temporary_path(path: &Path) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let file_name = path
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "atomic-write".to_string());
    path.with_file_name(format!(
        ".{file_name}.tmp-{}-{timestamp}",
        std::process::id()
    ))
}

#[cfg(not(windows))]
fn replace_path(from: &Path, to: &Path) -> Result<()> {
    fs::rename(from, to).with_context(|| format!("Failed to rename {:?} to {:?}", from, to))?;
    Ok(())
}

#[cfg(windows)]
fn replace_path(from: &Path, to: &Path) -> Result<()> {
    if !to.exists() {
        fs::rename(from, to).with_context(|| format!("Failed to rename {:?} to {:?}", from, to))?;
        return Ok(());
    }

    replace_file_windows(from, to)
}

#[cfg(windows)]
fn replace_file_windows(from: &Path, to: &Path) -> Result<()> {
    use std::ffi::OsStr;
    use std::iter;
    use std::os::windows::ffi::OsStrExt;

    unsafe extern "system" {
        fn ReplaceFileW(
            lpReplacedFileName: *const u16,
            lpReplacementFileName: *const u16,
            lpBackupFileName: *const u16,
            dwReplaceFlags: u32,
            lpExclude: *mut core::ffi::c_void,
            lpReserved: *mut core::ffi::c_void,
        ) -> i32;
    }

    fn wide(value: &OsStr) -> Vec<u16> {
        value.encode_wide().chain(iter::once(0)).collect()
    }

    let from_wide = wide(from.as_os_str());
    let to_wide = wide(to.as_os_str());
    let replaced = unsafe {
        ReplaceFileW(
            to_wide.as_ptr(),
            from_wide.as_ptr(),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if replaced == 0 {
        return Err(std::io::Error::last_os_error())
            .with_context(|| format!("Failed to replace {:?} with {:?}", to, from));
    }

    Ok(())
}

#[cfg(not(windows))]
fn sync_directory(path: &Path) -> Result<()> {
    let directory = File::open(path).with_context(|| format!("Failed to open {:?}", path))?;
    directory
        .sync_all()
        .with_context(|| format!("Failed to sync {:?}", path))?;
    Ok(())
}

#[cfg(windows)]
fn sync_directory(_path: &Path) -> Result<()> {
    Ok(())
}
