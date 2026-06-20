use crate::{
    bridge::{self, BridgePlan},
    config::{EngineConfig, Requirements},
    hardware::{self, HardwareReport},
    launcher::{self, LaunchPlan},
    plugins::{self, PluginManifest},
    trim::{self, TrimPlan, TrimProfile},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineRequest {
    pub target: Option<String>,
    pub requirements: Option<Requirements>,
    pub profile: Option<String>,
    pub runtime_mode: Option<String>,
    pub api_key_present: bool,
}

#[derive(Debug, Clone)]
pub struct EnginePlan {
    pub hardware: HardwareReport,
    pub trim: TrimPlan,
    pub bridge: BridgePlan,
    pub launch: LaunchPlan,
    pub plugins: Vec<PluginManifest>,
}

pub fn plan(mut config: EngineConfig, request: EngineRequest) -> Result<EnginePlan> {
    if let Some(target) = request.target {
        config.target = target;
    }
    if let Some(requirements) = request.requirements {
        config.requirements = requirements;
    }
    if let Some(profile) = request.profile {
        config.profile = profile;
    }

    let hardware = hardware::inspect()?;
    let profile = TrimProfile::from_name(&config.profile);
    let runtime_mode = request
        .runtime_mode
        .as_deref()
        .map(crate::runtime::RuntimeMode::from_name)
        .unwrap_or(config.runtime_mode);
    let trim = trim::plan(profile, runtime_mode);
    let api_key_marker = request.api_key_present.then_some("present");
    let bridge = bridge::plan(
        &hardware,
        &config.requirements,
        &config.cloud,
        api_key_marker,
    )?;
    let launch = launcher::prepare(&config.target, &config.browser)?;
    let plugins = plugins::load_manifests_with_policy(
        &PathBuf::from(&config.plugins_dir),
        &config.plugin_policy,
    )?;

    Ok(EnginePlan {
        hardware,
        trim,
        bridge,
        launch,
        plugins,
    })
}

impl EnginePlan {
    pub fn render_for_gui(&self) -> serde_json::Value {
        serde_json::json!({
            "hardware": {
                "os": &self.hardware.os,
                "arch": &self.hardware.arch,
                "total_ram_gb": self.hardware.total_ram_gb,
                "available_ram_gb": self.hardware.available_ram_gb,
                "cpu_threads": self.hardware.cpu_threads,
                "executable_sha256": &self.hardware.executable_sha256,
            },
            "trim": {
                "runtime_mode": format!("{:?}", self.trim.runtime_mode),
                "actions": self.trim.actions.iter().map(|action| {
                    serde_json::json!({
                        "id": action.id,
                        "label": action.label,
                        "description": action.description,
                        "program": action.program,
                        "args": &action.args,
                        "risk": format!("{:?}", action.risk),
                        "requires_admin": action.requires_admin,
                        "destructive": action.destructive,
                    })
                }).collect::<Vec<_>>(),
            },
            "bridge": {
                "provider": &self.bridge.provider,
                "endpoint": &self.bridge.endpoint,
                "needs_cloud": self.bridge.needs_cloud,
                "ram_deficit_gb": self.bridge.ram_deficit_gb,
                "vram_deficit_gb": self.bridge.vram_deficit_gb,
                "vcpu_deficit": self.bridge.vcpu_deficit,
                "cloud_storage_gb": self.bridge.cloud_storage_gb,
                "authenticated": self.bridge.authenticated,
            },
            "launch": {
                "target": &self.launch.target,
                "mode": format!("{:?}", self.launch.mode),
                "cache_dir": self.launch.cache_dir.as_ref().map(|path| path.display().to_string()),
                "chromium_heap_mb": self.launch.chromium_heap_mb,
            },
            "plugins": self.plugins.iter().map(|plugin| {
                serde_json::json!({
                    "name": &plugin.name,
                    "version": &plugin.version,
                    "hooks": &plugin.hooks,
                    "signed": plugin.signature.is_some(),
                })
            }).collect::<Vec<_>>(),
        })
    }
}
