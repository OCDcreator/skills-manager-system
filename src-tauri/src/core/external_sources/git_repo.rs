use anyhow::{anyhow, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
