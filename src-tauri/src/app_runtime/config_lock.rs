use fs2::FileExt;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const CONFIG_LOCK_FILE_NAME: &str = ".skills-manager-system.lock";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLockErrorKind {
    AlreadyHeld,
    Io,
}

#[derive(Debug)]
pub struct ConfigLockError {
    kind: ConfigLockErrorKind,
    lock_path: PathBuf,
    source: Option<std::io::Error>,
}

impl ConfigLockError {
    pub(crate) fn already_held(lock_path: PathBuf) -> Self {
        Self {
            kind: ConfigLockErrorKind::AlreadyHeld,
            lock_path,
            source: None,
        }
    }

    pub(crate) fn io(lock_path: PathBuf, source: std::io::Error) -> Self {
        Self {
            kind: ConfigLockErrorKind::Io,
            lock_path,
            source: Some(source),
        }
    }

    pub const fn kind(&self) -> ConfigLockErrorKind {
        self.kind
    }

    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }
}

impl Display for ConfigLockError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match (&self.kind, &self.source) {
            (ConfigLockErrorKind::AlreadyHeld, _) => {
                write!(
                    formatter,
                    "Configuration lock is already held at {:?}",
                    self.lock_path
                )
            }
            (ConfigLockErrorKind::Io, Some(source)) => {
                write!(
                    formatter,
                    "Failed to acquire configuration lock at {:?}: {source}",
                    self.lock_path
                )
            }
            (ConfigLockErrorKind::Io, None) => {
                write!(
                    formatter,
                    "Failed to acquire configuration lock at {:?}",
                    self.lock_path
                )
            }
        }
    }
}

impl Error for ConfigLockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
pub struct ConfigLockGuard {
    file: Option<File>,
}

impl Drop for ConfigLockGuard {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            let _ = file.unlock();
        }
    }
}

pub fn acquire_config_lock(config_dir: &Path) -> Result<ConfigLockGuard, ConfigLockError> {
    let lock_path = config_dir.join(CONFIG_LOCK_FILE_NAME);
    fs::create_dir_all(config_dir)
        .map_err(|source| ConfigLockError::io(lock_path.clone(), source))?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&lock_path)
        .map_err(|source| ConfigLockError::io(lock_path.clone(), source))?;

    match file.try_lock_exclusive() {
        Ok(()) => {}
        Err(source) if is_lock_contention(&source) => {
            return Err(ConfigLockError::already_held(lock_path));
        }
        Err(source) => {
            return Err(ConfigLockError::io(lock_path, source));
        }
    }

    file.set_len(0)
        .map_err(|source| ConfigLockError::io(lock_path.clone(), source))?;
    writeln!(file, "pid={}", std::process::id())
        .map_err(|source| ConfigLockError::io(lock_path, source))?;

    Ok(ConfigLockGuard { file: Some(file) })
}

fn is_lock_contention(error: &std::io::Error) -> bool {
    if error.kind() == std::io::ErrorKind::WouldBlock {
        return true;
    }

    #[cfg(windows)]
    {
        return matches!(error.raw_os_error(), Some(32 | 33));
    }

    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn advisory_lock_blocks_second_mutation_writer() {
        let config_dir = tempdir().unwrap();

        let _guard = acquire_config_lock(config_dir.path()).unwrap();
        let error = acquire_config_lock(config_dir.path()).unwrap_err();

        assert_eq!(error.kind(), ConfigLockErrorKind::AlreadyHeld);
    }

    #[test]
    fn advisory_lock_releases_after_guard_drop() {
        let config_dir = tempdir().unwrap();

        let guard = acquire_config_lock(config_dir.path()).unwrap();
        drop(guard);
        let _next_guard = acquire_config_lock(config_dir.path()).unwrap();
    }

    #[test]
    fn stale_lock_file_does_not_block_new_writer() {
        let config_dir = tempdir().unwrap();
        let lock_path = config_dir.path().join(CONFIG_LOCK_FILE_NAME);
        fs::write(&lock_path, "pid=old").unwrap();

        let _guard = acquire_config_lock(config_dir.path()).unwrap();

        assert!(lock_path.exists());
    }
}
