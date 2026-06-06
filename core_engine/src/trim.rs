use crate::runtime::RuntimeMode;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrimProfile {
    Standard,
    Aggressive,
    Recovery,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrimRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrimAction {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub program: &'static str,
    pub args: Vec<&'static str>,
    pub recovery: Option<RecoveryAction>,
    pub risk: TrimRisk,
    pub requires_admin: bool,
    pub destructive: bool,
    pub stress_only: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecoveryAction {
    pub label: &'static str,
    pub program: &'static str,
    pub args: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrimPlan {
    pub profile: TrimProfile,
    pub runtime_mode: RuntimeMode,
    pub actions: Vec<TrimAction>,
}

impl TrimProfile {
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "aggressive" | "profile_core_aggressive" => Self::Aggressive,
            "recovery" | "profile_recovery_baseline" => Self::Recovery,
            _ => Self::Standard,
        }
    }
}

pub fn plan(profile: TrimProfile, runtime_mode: RuntimeMode) -> TrimPlan {
    let mut actions = Vec::new();

    match profile {
        TrimProfile::Standard => standard_actions(&mut actions),
        TrimProfile::Aggressive => {
            standard_actions(&mut actions);
            aggressive_actions(&mut actions);
        }
        TrimProfile::Recovery => recovery_actions(&mut actions),
    }

    TrimPlan {
        profile,
        runtime_mode,
        actions,
    }
}

pub fn execute(plan: &TrimPlan) -> Result<()> {
    println!(
        "[TRIM]\nprofile={:?} runtime_mode={:?}",
        plan.profile, plan.runtime_mode
    );

    if plan.runtime_mode.allows_simulation() {
        println!("simulation_boundary=stress-debug");
    }

    if plan.runtime_mode.applies_real_changes() && needs_admin(&plan.actions) && !is_elevated()? {
        bail!("production trim requires elevated privileges for one or more planned actions");
    }

    for action in &plan.actions {
        if action.stress_only && !plan.runtime_mode.allows_simulation() {
            bail!("stress-only action {} cannot run outside stress/debug mode", action.id);
        }

        println!(
            "action={} risk={:?} destructive={} requires_admin={} command={} {}",
            action.id,
            action.risk,
            action.destructive,
            action.requires_admin,
            action.program,
            action.args.join(" ")
        );

        if plan.runtime_mode.applies_real_changes() {
            let status = Command::new(action.program)
                .args(&action.args)
                .status()
                .with_context(|| format!("failed to execute trim action {}", action.id))?;
            println!("status={}", status);
        }
    }

    Ok(())
}

fn needs_admin(actions: &[TrimAction]) -> bool {
    actions.iter().any(|action| action.requires_admin)
}

#[cfg(windows)]
fn is_elevated() -> Result<bool> {
    let status = Command::new("net")
        .arg("session")
        .status()
        .context("failed to check Windows elevation with net session")?;
    Ok(status.success())
}

#[cfg(not(windows))]
fn is_elevated() -> Result<bool> {
    let output = Command::new("id")
        .arg("-u")
        .output()
        .context("failed to check Unix elevation with id -u")?;
    Ok(String::from_utf8_lossy(&output.stdout).trim() == "0")
}

#[cfg(windows)]
fn standard_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "windows-stop-onedrive",
        label: "Stop OneDrive",
        description: "Stops OneDrive background sync to free IO, memory, and network pressure during a workload.",
        program: "taskkill",
        args: vec!["/IM", "OneDrive.exe", "/F"],
        recovery: None,
        risk: TrimRisk::Low,
        requires_admin: false,
        destructive: false,
        stress_only: false,
    });
    actions.push(TrimAction {
        id: "windows-stop-sysmain",
        label: "Stop SysMain",
        description: "Stops Windows SysMain prefetch service to reduce background disk and memory pressure.",
        program: "net",
        args: vec!["stop", "SysMain", "/y"],
        recovery: Some(RecoveryAction {
            label: "Start SysMain",
            program: "net",
            args: vec!["start", "SysMain"],
        }),
        risk: TrimRisk::Medium,
        requires_admin: true,
        destructive: false,
        stress_only: false,
    });
}

#[cfg(not(windows))]
fn standard_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "linux-stop-tracker-miner",
        label: "Stop tracker miner",
        description: "Stops desktop file indexing to reduce IO and CPU pressure during a workload.",
        program: "systemctl",
        args: vec!["stop", "tracker-miner-fs-3.service"],
        recovery: Some(RecoveryAction {
            label: "Start tracker miner",
            program: "systemctl",
            args: vec!["start", "tracker-miner-fs-3.service"],
        }),
        risk: TrimRisk::Low,
        requires_admin: true,
        destructive: false,
        stress_only: false,
    });
}

#[cfg(windows)]
fn aggressive_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "windows-stop-explorer",
        label: "Stop Explorer shell",
        description: "Stops the desktop shell to reclaim memory. Intended only for controlled heavy workloads.",
        program: "taskkill",
        args: vec!["/IM", "explorer.exe", "/F"],
        recovery: Some(RecoveryAction {
            label: "Restart Explorer shell",
            program: "explorer.exe",
            args: vec![],
        }),
        risk: TrimRisk::High,
        requires_admin: false,
        destructive: false,
        stress_only: false,
    });
}

#[cfg(not(windows))]
fn aggressive_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "linux-stop-display-manager",
        label: "Stop display manager",
        description: "Stops graphical session manager to reclaim resources. Intended for controlled server-style runs.",
        program: "systemctl",
        args: vec!["stop", "display-manager"],
        recovery: Some(RecoveryAction {
            label: "Start display manager",
            program: "systemctl",
            args: vec!["start", "display-manager"],
        }),
        risk: TrimRisk::High,
        requires_admin: true,
        destructive: false,
        stress_only: false,
    });
}

#[cfg(windows)]
fn recovery_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "windows-start-sysmain",
        label: "Start SysMain",
        description: "Restores Windows SysMain service after a workload.",
        program: "net",
        args: vec!["start", "SysMain"],
        recovery: None,
        risk: TrimRisk::Low,
        requires_admin: true,
        destructive: false,
        stress_only: false,
    });
    actions.push(TrimAction {
        id: "windows-start-explorer",
        label: "Restart Explorer shell",
        description: "Restores the Windows desktop shell after aggressive trim.",
        program: "explorer.exe",
        args: vec![],
        recovery: None,
        risk: TrimRisk::Low,
        requires_admin: false,
        destructive: false,
        stress_only: false,
    });
}

#[cfg(not(windows))]
fn recovery_actions(actions: &mut Vec<TrimAction>) {
    actions.push(TrimAction {
        id: "linux-start-tracker-miner",
        label: "Start tracker miner",
        description: "Restores desktop file indexing after a workload.",
        program: "systemctl",
        args: vec!["start", "tracker-miner-fs-3.service"],
        recovery: None,
        risk: TrimRisk::Low,
        requires_admin: true,
        destructive: false,
        stress_only: false,
    });
}
