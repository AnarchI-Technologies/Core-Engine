use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::config::PluginPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub hooks: Vec<String>,
    pub entry: Option<String>,
    pub signature: Option<String>,
}

const ALLOWED_HOOKS: &[&str] = &[
    "pre_inspect",
    "post_inspect",
    "pre_trim",
    "post_trim",
    "pre_bridge",
    "post_bridge",
    "pre_run",
    "post_run",
];

pub fn load_manifests(dir: &Path) -> Result<Vec<PluginManifest>> {
    load_manifests_with_policy(dir, &PluginPolicy::default())
}

pub fn load_manifests_with_policy(
    dir: &Path,
    policy: &PluginPolicy,
) -> Result<Vec<PluginManifest>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let data = fs::read_to_string(&path)
            .with_context(|| format!("failed to read plugin manifest {}", path.display()))?;
        let manifest: PluginManifest = serde_json::from_str(&data)
            .with_context(|| format!("failed to parse plugin manifest {}", path.display()))?;
        validate_manifest(&manifest, policy)
            .with_context(|| format!("invalid plugin manifest {}", path.display()))?;
        manifests.push(manifest);
    }

    manifests.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(manifests)
}

pub fn validate_manifest(manifest: &PluginManifest, policy: &PluginPolicy) -> Result<()> {
    if manifest.name.trim().is_empty() {
        bail!("plugin name is required");
    }
    if manifest.version.trim().is_empty() {
        bail!("plugin version is required");
    }
    for hook in &manifest.hooks {
        if !ALLOWED_HOOKS.contains(&hook.as_str()) {
            bail!("unsupported plugin hook {}", hook);
        }
    }
    if policy.require_signatures && manifest.signature.is_none() {
        bail!(
            "plugin {} is unsigned and signatures are required",
            manifest.name
        );
    }
    if !policy.allow_native_sidecars && manifest.entry.is_some() {
        bail!(
            "plugin {} declares a native entry but sidecars are disabled",
            manifest.name
        );
    }
    Ok(())
}

pub fn list_and_print(dir: PathBuf) -> Result<()> {
    let manifests = load_manifests(&dir)?;
    println!("[PLUGINS]\ndir={} count={}", dir.display(), manifests.len());
    for manifest in manifests {
        println!(
            "{}@{} hooks={} signed={}",
            manifest.name,
            manifest.version,
            manifest.hooks.join(","),
            manifest.signature.is_some()
        );
    }
    Ok(())
}

pub fn print_hook_summary(manifests: &[PluginManifest], hook: &str) {
    let count = manifests
        .iter()
        .filter(|manifest| manifest.hooks.iter().any(|candidate| candidate == hook))
        .count();
    println!("[PLUGIN-HOOK]\nhook={} eligible={}", hook, count);
}
