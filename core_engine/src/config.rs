use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::runtime::RuntimeMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    #[serde(default = "default_runtime_mode")]
    pub runtime_mode: RuntimeMode,
    pub profile: String,
    pub target: String,
    pub requirements: Requirements,
    pub browser: BrowserConfig,
    pub cloud: CloudConfig,
    pub plugins_dir: String,
    #[serde(default)]
    pub plugin_policy: PluginPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub ram_gb: f64,
    pub vram_gb: f64,
    pub vcpu: u16,
    pub cloud_storage_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub chromium_heap_mb: u32,
    pub persistent_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub active_provider: String,
    pub providers: Vec<CloudProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProvider {
    pub name: String,
    pub endpoint: String,
    pub priority: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPolicy {
    pub require_signatures: bool,
    pub allow_native_sidecars: bool,
    pub allow_stress_only_plugins: bool,
}

impl Default for PluginPolicy {
    fn default() -> Self {
        Self {
            require_signatures: false,
            allow_native_sidecars: false,
            allow_stress_only_plugins: false,
        }
    }
}

impl EngineConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let data = fs::read_to_string(path)
            .with_context(|| format!("failed to read config {}", path.display()))?;
        serde_json::from_str(&data).with_context(|| format!("failed to parse config {}", path.display()))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create config directory {}", parent.display()))?;
        }
        let data = serde_json::to_string_pretty(self).context("failed to serialize config")?;
        fs::write(path, data).with_context(|| format!("failed to write config {}", path.display()))
    }
}

fn default_runtime_mode() -> RuntimeMode {
    RuntimeMode::DryRun
}
