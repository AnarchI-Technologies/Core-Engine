use anyhow::{Context, Result};
use serde_json::Value;
use std::{fs, path::PathBuf};

pub fn print_manifest(path: PathBuf) -> Result<()> {
    let data = fs::read_to_string(&path)
        .with_context(|| format!("failed to read marketplace manifest {}", path.display()))?;
    let value: Value = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse marketplace manifest {}", path.display()))?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
