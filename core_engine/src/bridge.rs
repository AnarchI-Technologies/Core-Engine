use crate::{
    config::{CloudConfig, Requirements},
    hardware::HardwareReport,
};
use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct BridgePlan {
    pub provider: String,
    pub endpoint: String,
    pub needs_cloud: bool,
    pub ram_deficit_gb: f64,
    pub vram_deficit_gb: f64,
    pub vcpu_deficit: i32,
    pub cloud_storage_gb: f64,
    pub authenticated: bool,
}

pub fn plan(
    hardware: &HardwareReport,
    requirements: &Requirements,
    cloud: &CloudConfig,
    api_key: Option<&str>,
) -> Result<BridgePlan> {
    let provider = cloud
        .providers
        .iter()
        .find(|provider| provider.name == cloud.active_provider)
        .or_else(|| {
            cloud
                .providers
                .iter()
                .min_by_key(|provider| provider.priority)
        });

    let Some(provider) = provider else {
        bail!("no cloud providers configured");
    };

    let ram_deficit_gb = (requirements.ram_gb - hardware.available_ram_gb).max(0.0);
    let vram_deficit_gb = (requirements.vram_gb - local_vram_estimate_gb()).max(0.0);
    let vcpu_deficit = (requirements.vcpu as i32 - hardware.cpu_threads as i32).max(0);
    let needs_cloud = ram_deficit_gb > 0.0
        || vram_deficit_gb > 0.0
        || vcpu_deficit > 0
        || requirements.cloud_storage_gb > 0.0;

    Ok(BridgePlan {
        provider: provider.name.clone(),
        endpoint: provider.endpoint.clone(),
        needs_cloud,
        ram_deficit_gb,
        vram_deficit_gb,
        vcpu_deficit,
        cloud_storage_gb: requirements.cloud_storage_gb,
        authenticated: api_key.is_some_and(|key| !key.trim().is_empty()),
    })
}

impl BridgePlan {
    pub fn render(&self) -> String {
        if !self.needs_cloud {
            return "[BRIDGE]\nlocal resources satisfy requested workload".to_string();
        }

        format!(
            "[BRIDGE]\nprovider={} endpoint={}\nauthenticated={}\nrequest_ram_gb={:.2} request_vram_gb={:.2} request_vcpu={} request_cloud_storage_gb={:.2}",
            self.provider,
            self.endpoint,
            self.authenticated,
            self.ram_deficit_gb,
            self.vram_deficit_gb,
            self.vcpu_deficit,
            self.cloud_storage_gb
        )
    }
}

fn local_vram_estimate_gb() -> f64 {
    2.0
}
