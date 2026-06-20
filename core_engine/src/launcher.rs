use crate::{cache, config::BrowserConfig, runtime::RuntimeMode};
use anyhow::{Context, Result};
use std::{path::PathBuf, process::Command};
use url::Url;

#[derive(Debug, Clone)]
pub enum LaunchMode {
    Url,
    Executable,
}

#[derive(Debug, Clone)]
pub struct LaunchPlan {
    pub target: String,
    pub mode: LaunchMode,
    pub cache_dir: Option<PathBuf>,
    pub chromium_heap_mb: u32,
}

pub fn prepare(target: &str, browser: &BrowserConfig) -> Result<LaunchPlan> {
    let mode = if is_http_url(target) {
        LaunchMode::Url
    } else {
        LaunchMode::Executable
    };
    let cache_dir = if matches!(mode, LaunchMode::Url) && browser.persistent_cache {
        Some(cache::cache_dir_for(target)?)
    } else {
        None
    };

    Ok(LaunchPlan {
        target: target.to_string(),
        mode,
        cache_dir,
        chromium_heap_mb: browser.chromium_heap_mb,
    })
}

pub fn launch(plan: &LaunchPlan) -> Result<()> {
    launch_with_mode(plan, RuntimeMode::Production)
}

pub fn launch_with_mode(plan: &LaunchPlan, runtime_mode: RuntimeMode) -> Result<()> {
    if !runtime_mode.applies_real_changes() {
        println!(
            "launch=dry-run runtime_mode={:?} target={}",
            runtime_mode, plan.target
        );
        return Ok(());
    }

    match plan.mode {
        LaunchMode::Url => {
            println!("launch=system-browser target={}", plan.target);
            open_url(&plan.target)
        }
        LaunchMode::Executable => {
            println!("launch=executable target={}", plan.target);
            Command::new(&plan.target)
                .spawn()
                .with_context(|| format!("failed to launch {}", plan.target))?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BrowserConfig;

    fn browser_config() -> BrowserConfig {
        BrowserConfig {
            chromium_heap_mb: 1024,
            persistent_cache: false,
        }
    }

    #[test]
    fn prepare_classifies_urls_without_launching() {
        let plan = prepare("https://anarchi.example", &browser_config()).unwrap();

        assert!(matches!(plan.mode, LaunchMode::Url));
        assert!(plan.cache_dir.is_none());
    }

    #[test]
    fn dry_run_launch_is_side_effect_free() {
        let plan = prepare("definitely-not-a-real-binary.exe", &browser_config()).unwrap();

        launch_with_mode(&plan, RuntimeMode::DryRun).unwrap();
    }

    #[test]
    fn stress_launch_is_side_effect_free() {
        let plan = prepare("https://anarchi.example", &browser_config()).unwrap();

        launch_with_mode(&plan, RuntimeMode::StressTest).unwrap();
    }
}

impl LaunchPlan {
    pub fn render(&self) -> String {
        format!(
            "[LAUNCH]\nmode={:?} target={}\ncache_dir={}\nchromium_heap_mb={}",
            self.mode,
            self.target,
            self.cache_dir
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "none".to_string()),
            self.chromium_heap_mb
        )
    }
}

fn is_http_url(target: &str) -> bool {
    Url::parse(target)
        .map(|url| matches!(url.scheme(), "http" | "https"))
        .unwrap_or(false)
}

#[cfg(windows)]
fn open_url(target: &str) -> Result<()> {
    Command::new("cmd")
        .args(["/C", "start", "", target])
        .spawn()
        .context("failed to open URL through Windows shell")?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn open_url(target: &str) -> Result<()> {
    Command::new("open")
        .arg(target)
        .spawn()
        .context("failed to open URL")?;
    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn open_url(target: &str) -> Result<()> {
    Command::new("xdg-open")
        .arg(target)
        .spawn()
        .context("failed to open URL")?;
    Ok(())
}
