use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::{env, fs::File, io::Read};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct HardwareReport {
    pub os: String,
    pub arch: String,
    pub total_ram_gb: f64,
    pub available_ram_gb: f64,
    pub cpu_threads: usize,
    pub executable_sha256: String,
}

pub fn inspect() -> Result<HardwareReport> {
    let mut system = System::new_all();
    system.refresh_all();

    Ok(HardwareReport {
        os: env::consts::OS.to_string(),
        arch: env::consts::ARCH.to_string(),
        total_ram_gb: bytes_to_gb(system.total_memory()),
        available_ram_gb: bytes_to_gb(system.available_memory()),
        cpu_threads: system.cpus().len(),
        executable_sha256: current_exe_hash()?,
    })
}

impl HardwareReport {
    pub fn render(&self) -> String {
        format!(
            "[INSPECT]\nos={} arch={}\nram_total_gb={:.2} ram_available_gb={:.2} cpu_threads={}\nexe_sha256={}",
            self.os,
            self.arch,
            self.total_ram_gb,
            self.available_ram_gb,
            self.cpu_threads,
            self.executable_sha256
        )
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

fn current_exe_hash() -> Result<String> {
    let path = env::current_exe().context("failed to resolve current executable")?;
    let mut file =
        File::open(&path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
