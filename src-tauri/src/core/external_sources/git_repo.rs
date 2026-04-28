use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::hash::sha256_hex;
use crate::core::skills::identity::canonicalize_repo_relative_path;

pub fn normalize_github_repo_url(remote: &str) -> Result<String> {
    let remote = remote.trim();
    if remote.is_empty() {
        return Err(anyhow!("GitHub remote cannot be empty"));
    }

    if let Some(path) = remote.strip_prefix("https://") {
        return normalize_https_path(path);
    }

    if let Some(path) = remote.strip_prefix("ssh://git@") {
        return normalize_ssh_url_path(path);
    }

    if let Some(path) = remote.strip_prefix("git@") {
        return normalize_ssh_path(path);
    }

    Err(anyhow!("Only GitHub HTTPS and SSH remotes are supported"))
}

pub fn ensure_cached_repo(cache_root: &Path, source_id: &str, repo_url: &str) -> Result<PathBuf> {
    let source_id = validate_source_id(source_id)?;

    let repo_dir = cache_root.join(source_id).join("repo");
    if repo_dir.exists() && !repo_dir.is_dir() {
        bail!("Cached repo path is not a directory: {}", repo_dir.display());
    }

    fs::create_dir_all(repo_dir.parent().context("Cached repo parent is missing")?)
        .with_context(|| format!("Failed to create cache root {}", cache_root.display()))?;

    if !repo_dir.join(".git").exists() {
        fs::create_dir_all(&repo_dir)
            .with_context(|| format!("Failed to create repo dir {}", repo_dir.display()))?;
        run_git(
            Command::new("git")
                .arg("init")
                .arg(&repo_dir)
                .env("GIT_TERMINAL_PROMPT", "0")
                .env("LC_ALL", "C"),
        )
        .with_context(|| format!("Failed to initialize cached repo {}", repo_dir.display()))?;
    }

    sync_origin_remote(&repo_dir, repo_url)?;
    run_git(git_cmd(&repo_dir).args([
        "fetch",
        "--prune",
        "origin",
        "+refs/heads/*:refs/remotes/origin/*",
    ]))
    .with_context(|| format!("Failed to fetch {}", repo_url))?;
    let _ = run_git(git_cmd(&repo_dir).args(["remote", "set-head", "origin", "--auto"]));

    Ok(repo_dir)
}

pub fn read_default_branch(repo_dir: &Path) -> Result<String> {
    if run_git(git_cmd(repo_dir).args(["remote", "set-head", "origin", "--auto"])).is_err() {
        // Some remotes may not advertise HEAD, but symbolic-ref can still succeed from prior fetches.
    }

    if let Ok(head_ref) = run_git(git_cmd(repo_dir).args([
        "symbolic-ref",
        "refs/remotes/origin/HEAD",
        "--short",
    ])) {
        return head_ref
            .trim()
            .strip_prefix("origin/")
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("Unexpected origin HEAD ref: {head_ref}"));
    }

    let symref_output = run_git(git_cmd(repo_dir).args(["ls-remote", "--symref", "origin", "HEAD"]))
        .context("Failed to resolve origin HEAD")?;
    for line in symref_output.lines() {
        if let Some(rest) = line.strip_prefix("ref: refs/heads/") {
            let (branch, target) = rest
                .split_once('\t')
                .ok_or_else(|| anyhow!("Unexpected ls-remote HEAD output: {line}"))?;
            if target == "HEAD" {
                return Ok(branch.to_string());
            }
        }
    }

    Err(anyhow!("Failed to resolve origin HEAD"))
}

pub fn read_head_commit(repo_dir: &Path, branch: &str) -> Result<String> {
    let branch = branch.trim().trim_start_matches("origin/");
    if branch.is_empty() {
        bail!("Branch cannot be empty");
    }

    run_git(git_cmd(repo_dir).args([
        "rev-parse",
        &format!("refs/remotes/origin/{branch}^{{commit}}"),
    ]))
    .with_context(|| format!("Failed to read commit for origin/{branch}"))
}

pub fn fingerprint_variant_at_ref(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
) -> Result<Option<String>> {
    let variant_path = canonicalize_repo_relative_path(variant_path)?;
    let mut entries = list_variant_blob_entries(repo_dir, git_ref, &variant_path)?;
    if entries.is_empty() {
        return Ok(None);
    }

    entries.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

    let mut serialized = Vec::new();
    for entry in entries {
        if entry.relative_path == ".skills-manager-source.json" {
            continue;
        }

        let blob_bytes = run_git_bytes(git_cmd(repo_dir).args(["cat-file", "blob", &entry.object_id]))
            .with_context(|| format!("Failed to read blob {}", entry.object_id))?;
        let file_hash = sha256_hex(&blob_bytes);
        serialized.extend_from_slice(entry.relative_path.as_bytes());
        serialized.push(b'\n');
        serialized.extend_from_slice(file_hash.as_bytes());
        serialized.push(b'\n');
    }

    Ok(Some(format!("sha256:{}", sha256_hex(&serialized))))
}

fn normalize_https_path(path: &str) -> Result<String> {
    let (host, repo_path) = path
        .split_once('/')
        .ok_or_else(|| anyhow!("GitHub HTTPS remote must include owner and repository"))?;
    if !host.eq_ignore_ascii_case("github.com") {
        return Err(anyhow!("Remote host must be github.com"));
    }

    normalize_repo_path(repo_path)
}

fn normalize_ssh_path(path: &str) -> Result<String> {
    let (host, repo_path) = path
        .split_once(':')
        .ok_or_else(|| anyhow!("GitHub SSH remote must include owner and repository"))?;
    if !host.eq_ignore_ascii_case("github.com") {
        return Err(anyhow!("Remote host must be github.com"));
    }

    normalize_repo_path(repo_path)
}

fn normalize_ssh_url_path(path: &str) -> Result<String> {
    let (host, repo_path) = path
        .split_once('/')
        .ok_or_else(|| anyhow!("GitHub SSH remote must include owner and repository"))?;
    if !host.eq_ignore_ascii_case("github.com") {
        return Err(anyhow!("Remote host must be github.com"));
    }

    normalize_repo_path(repo_path)
}

fn normalize_repo_path(path: &str) -> Result<String> {
    let path = path.trim().trim_end_matches('/');
    if path.is_empty() || path.contains('?') || path.contains('#') {
        return Err(anyhow!("GitHub remote path is invalid"));
    }

    let mut segments = path.split('/');
    let owner = segments
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("GitHub remote owner is missing"))?;
    let repo = segments
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("GitHub remote repository is missing"))?;
    if segments.next().is_some() {
        return Err(anyhow!("GitHub remote must target exactly one repository"));
    }

    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    if repo.is_empty() {
        return Err(anyhow!("GitHub remote repository is missing"));
    }

    Ok(format!(
        "github.com/{}/{}",
        owner.to_lowercase(),
        repo.to_lowercase()
    ))
}

fn validate_source_id(source_id: &str) -> Result<&str> {
    let source_id = source_id.trim();
    if source_id.is_empty() {
        bail!("Source id cannot be empty");
    }
    if source_id == "." || source_id == ".." || source_id.contains('/') || source_id.contains('\\') {
        bail!("Source id must be a single path-safe segment");
    }
    Ok(source_id)
}

fn sync_origin_remote(repo_dir: &Path, repo_url: &str) -> Result<()> {
    match run_git(git_cmd(repo_dir).args(["remote", "get-url", "origin"])) {
        Ok(existing) if existing.trim() == repo_url.trim() => Ok(()),
        Ok(_) => {
            run_git(git_cmd(repo_dir).args(["remote", "set-url", "origin", repo_url]))?;
            Ok(())
        }
        Err(_) => {
            run_git(git_cmd(repo_dir).args(["remote", "add", "origin", repo_url]))?;
            Ok(())
        }
    }
}

fn list_variant_blob_entries(
    repo_dir: &Path,
    git_ref: &str,
    variant_path: &str,
) -> Result<Vec<VariantBlobEntry>> {
    let output = run_git_bytes(git_cmd(repo_dir).args([
        "ls-tree",
        "-r",
        "--full-tree",
        "-z",
        git_ref,
        "--",
        variant_path,
    ]))
    .with_context(|| format!("Failed to list tree for {git_ref}:{variant_path}"))?;

    let mut entries = Vec::new();
    for chunk in output.split(|byte| *byte == 0) {
        if chunk.is_empty() {
            continue;
        }

        let tab_index = chunk
            .iter()
            .position(|byte| *byte == b'\t')
            .ok_or_else(|| anyhow!("Malformed ls-tree output"))?;
        let header = std::str::from_utf8(&chunk[..tab_index]).context("Malformed tree header")?;
        let path = std::str::from_utf8(&chunk[tab_index + 1..]).context("Malformed tree path")?;

        let mut header_parts = header.split_whitespace();
        let _mode = header_parts.next();
        let object_type = header_parts.next();
        let object_id = header_parts.next();
        if object_type != Some("blob") {
            continue;
        }

        let relative_path = if path == variant_path {
            PathBuf::from(path)
                .file_name()
                .map(|value| value.to_string_lossy().to_string())
                .ok_or_else(|| anyhow!("Invalid variant path: {variant_path}"))?
        } else {
            path.strip_prefix(&format!("{variant_path}/"))
                .ok_or_else(|| anyhow!("Tree entry escaped variant path: {path}"))?
                .to_string()
        };

        entries.push(VariantBlobEntry {
            relative_path,
            object_id: object_id
                .ok_or_else(|| anyhow!("Missing blob object id"))?
                .to_string(),
        });
    }

    Ok(entries)
}

fn git_cmd(repo_path: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_path)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("LC_ALL", "C");
    cmd
}

fn run_git(cmd: &mut Command) -> Result<String> {
    let output = cmd.output().context("Failed to execute git")?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        Err(anyhow!(stderr))
    }
}

fn run_git_bytes(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd.output().context("Failed to execute git")?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end()
            .to_string();
        Err(anyhow!(stderr))
    }
}

struct VariantBlobEntry {
    relative_path: String,
    object_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn normalize_github_repo_url_coalesces_https_and_ssh_variants() {
        let https = normalize_github_repo_url("https://github.com/OCDcreator/Skills-Manager-System")
            .unwrap();
        let https_git =
            normalize_github_repo_url("https://github.com/OCDcreator/Skills-Manager-System.git")
                .unwrap();
        let ssh = normalize_github_repo_url("git@github.com:OCDcreator/Skills-Manager-System")
            .unwrap();
        let ssh_url =
            normalize_github_repo_url("ssh://git@github.com/OCDcreator/Skills-Manager-System.git")
                .unwrap();

        assert_eq!(https, "github.com/ocdcreator/skills-manager-system");
        assert_eq!(https_git, https);
        assert_eq!(ssh, https);
        assert_eq!(ssh_url, https);
    }

    #[test]
    fn normalize_github_repo_url_rejects_non_github_remote() {
        assert!(normalize_github_repo_url("https://gitlab.com/OCDcreator/example").is_err());
        assert!(normalize_github_repo_url("git@example.com:OCDcreator/example").is_err());
    }

    #[test]
    fn read_default_branch_resolves_origin_head_after_cache_fetch() {
        let temp = tempdir().unwrap();
        let remote_dir = temp.path().join("remote.git");
        let work_dir = temp.path().join("work");
        let cache_root = temp.path().join("cache");

        run_git(Command::new("git").arg("init").arg("--bare").arg(&remote_dir)).unwrap();
        run_git(Command::new("git").arg("clone").arg(&remote_dir).arg(&work_dir)).unwrap();
        run_git(git_cmd(&work_dir).args(["checkout", "-b", "main"])).unwrap();
        run_git(git_cmd(&work_dir).args(["config", "user.email", "test@example.com"])).unwrap();
        run_git(git_cmd(&work_dir).args(["config", "user.name", "Test User"])).unwrap();
        fs::create_dir_all(work_dir.join("dist/agents/.agents/skills/impeccable")).unwrap();
        fs::write(
            work_dir.join("dist/agents/.agents/skills/impeccable/SKILL.md"),
            "# Impeccable\n",
        )
        .unwrap();
        run_git(git_cmd(&work_dir).args(["add", "."])).unwrap();
        run_git(git_cmd(&work_dir).args(["commit", "-m", "initial"])).unwrap();
        run_git(git_cmd(&work_dir).args(["push", "-u", "origin", "main"])).unwrap();
        run_git(
            Command::new("git")
                .arg("--git-dir")
                .arg(&remote_dir)
                .args(["symbolic-ref", "HEAD", "refs/heads/main"]),
        )
        .unwrap();

        let repo_dir =
            ensure_cached_repo(&cache_root, "src_test", remote_dir.to_string_lossy().as_ref())
                .unwrap();

        assert_eq!(read_default_branch(&repo_dir).unwrap(), "main");
        assert_eq!(read_head_commit(&repo_dir, "main").unwrap().len(), 40);
    }

    #[test]
    fn fingerprint_variant_at_ref_reads_commit_blobs_not_worktree_bytes() {
        let temp = tempdir().unwrap();
        let repo_dir = temp.path().join("repo");

        run_git(Command::new("git").arg("init").arg(&repo_dir)).unwrap();
        run_git(git_cmd(&repo_dir).args(["config", "user.email", "test@example.com"])).unwrap();
        run_git(git_cmd(&repo_dir).args(["config", "user.name", "Test User"])).unwrap();

        let variant_dir = repo_dir.join("dist/agents/.agents/skills/impeccable");
        fs::create_dir_all(&variant_dir).unwrap();
        fs::write(variant_dir.join("SKILL.md"), "line one\nline two\n").unwrap();
        fs::write(variant_dir.join("notes.txt"), "alpha\n").unwrap();
        run_git(git_cmd(&repo_dir).args(["add", "."])).unwrap();
        run_git(git_cmd(&repo_dir).args(["commit", "-m", "initial"])).unwrap();

        let head = run_git(git_cmd(&repo_dir).args(["rev-parse", "HEAD"])).unwrap();
        let before = fingerprint_variant_at_ref(
            &repo_dir,
            &head,
            "dist/agents/.agents/skills/impeccable",
        )
        .unwrap()
        .unwrap();

        fs::write(variant_dir.join("SKILL.md"), "line one\r\nline two\r\n").unwrap();

        let after = fingerprint_variant_at_ref(
            &repo_dir,
            &head,
            "dist/agents/.agents/skills/impeccable",
        )
        .unwrap()
        .unwrap();

        assert_eq!(before, after);
    }

    #[test]
    fn ensure_cached_repo_rejects_parent_traversal_source_id() {
        let temp = tempdir().unwrap();

        let error =
            ensure_cached_repo(temp.path(), "../other", "https://github.com/example/repo.git")
                .unwrap_err();

        assert!(error
            .to_string()
            .contains("Source id must be a single path-safe segment"));
    }

    #[test]
    fn ensure_cached_repo_rejects_source_id_with_forward_slash() {
        let temp = tempdir().unwrap();

        let error =
            ensure_cached_repo(temp.path(), "foo/bar", "https://github.com/example/repo.git")
                .unwrap_err();

        assert!(error
            .to_string()
            .contains("Source id must be a single path-safe segment"));
    }

    #[test]
    fn ensure_cached_repo_rejects_source_id_with_backslash() {
        let temp = tempdir().unwrap();

        let error =
            ensure_cached_repo(temp.path(), r"foo\bar", "https://github.com/example/repo.git")
                .unwrap_err();

        assert!(error
            .to_string()
            .contains("Source id must be a single path-safe segment"));
    }
}
