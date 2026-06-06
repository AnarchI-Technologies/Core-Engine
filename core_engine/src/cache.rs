use anyhow::{Context, Result};
use directories::ProjectDirs;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use url::Url;

pub fn cache_dir_for(target: &str) -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("io", "AnarchI", "AnarchI Core")
        .context("failed to resolve platform cache directory")?;
    let key = cache_key(target);
    let digest = Sha256::digest(key.as_bytes());
    let suffix = format!("{:x}", digest);
    let path = project_dirs.cache_dir().join("browser").join(&suffix[..16]);
    fs::create_dir_all(&path).with_context(|| format!("failed to create {}", path.display()))?;
    Ok(path)
}

fn cache_key(target: &str) -> String {
    if let Ok(url) = Url::parse(target) {
        let port = url.port().map(|port| format!(":{port}")).unwrap_or_default();
        let mut path = url.path().to_string();
        if !path.ends_with('/') {
            if let Some((prefix, _)) = path.rsplit_once('/') {
                path = format!("{prefix}/");
            }
        }
        return format!("{}://{}{}{}", url.scheme(), url.host_str().unwrap_or_default(), port, path);
    }

    target.to_string()
}
