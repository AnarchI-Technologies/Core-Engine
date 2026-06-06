use anyhow::Result;
use anarchi_core::{
    bridge,
    config::{EngineConfig, Requirements},
    hardware, launcher, package, plugins,
    runtime::RuntimeMode,
    trim::{self, TrimProfile},
};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "anarchi-core")]
#[command(version)]
#[command(about = "Rust resource trim and cloud bridge engine.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Inspect {
        #[arg(long, default_value = "config/default.json")]
        config: PathBuf,
    },
    Trim {
        #[arg(long, value_enum, default_value_t = ProfileArg::Standard)]
        profile: ProfileArg,
        #[arg(long, value_enum, default_value_t = RuntimeArg::DryRun)]
        runtime_mode: RuntimeArg,
        #[arg(long)]
        apply: bool,
    },
    Bridge {
        #[arg(long)]
        ram_gb: f64,
        #[arg(long)]
        vram_gb: f64,
        #[arg(long, default_value_t = 4)]
        vcpu: u16,
        #[arg(long, default_value_t = 20.0)]
        cloud_storage_gb: f64,
        #[arg(long, env = "ANARCHI_API_KEY")]
        api_key: Option<String>,
        #[arg(long, default_value = "config/default.json")]
        config: PathBuf,
    },
    Run {
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        ram_gb: Option<f64>,
        #[arg(long)]
        vram_gb: Option<f64>,
        #[arg(long)]
        vcpu: Option<u16>,
        #[arg(long)]
        cloud_storage_gb: Option<f64>,
        #[arg(long, env = "ANARCHI_API_KEY")]
        api_key: Option<String>,
        #[arg(long, value_enum)]
        profile: Option<ProfileArg>,
        #[arg(long, value_enum)]
        runtime_mode: Option<RuntimeArg>,
        #[arg(long)]
        apply_trim: bool,
        #[arg(long, default_value = "config/default.json")]
        config: PathBuf,
    },
    Plugins {
        #[command(subcommand)]
        command: PluginCommand,
    },
    Package {
        #[command(subcommand)]
        command: PackageCommand,
    },
}

#[derive(Subcommand)]
enum PluginCommand {
    List {
        #[arg(long, default_value = "plugins")]
        dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum PackageCommand {
    Manifest {
        #[arg(long, default_value = "marketplace/marketplace.json")]
        marketplace: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ProfileArg {
    Standard,
    Aggressive,
    Recovery,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum RuntimeArg {
    Production,
    DryRun,
    StressTest,
    Debug,
}

impl From<ProfileArg> for TrimProfile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::Standard => TrimProfile::Standard,
            ProfileArg::Aggressive => TrimProfile::Aggressive,
            ProfileArg::Recovery => TrimProfile::Recovery,
        }
    }
}

impl From<RuntimeArg> for RuntimeMode {
    fn from(value: RuntimeArg) -> Self {
        match value {
            RuntimeArg::Production => RuntimeMode::Production,
            RuntimeArg::DryRun => RuntimeMode::DryRun,
            RuntimeArg::StressTest => RuntimeMode::StressTest,
            RuntimeArg::Debug => RuntimeMode::Debug,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Inspect { config } => {
            let cfg = EngineConfig::load(config)?;
            let report = hardware::inspect()?;
            println!("{}", report.render());
            println!("config_profile={}", cfg.profile);
        }
        Command::Trim {
            profile,
            runtime_mode,
            apply,
        } => {
            let runtime_mode = if apply {
                RuntimeMode::Production
            } else {
                runtime_mode.into()
            };
            let plan = trim::plan(profile.into(), runtime_mode);
            trim::execute(&plan)?;
        }
        Command::Bridge {
            ram_gb,
            vram_gb,
            vcpu,
            cloud_storage_gb,
            api_key,
            config,
        } => {
            let cfg = EngineConfig::load(config)?;
            let req = Requirements {
                ram_gb,
                vram_gb,
                vcpu,
                cloud_storage_gb,
            };
            let plan = bridge::plan(&hardware::inspect()?, &req, &cfg.cloud, api_key.as_deref())?;
            println!("{}", plan.render());
        }
        Command::Run {
            target,
            ram_gb,
            vram_gb,
            vcpu,
            cloud_storage_gb,
            api_key,
            profile,
            runtime_mode,
            apply_trim,
            config,
        } => run(
            config,
            target,
            ram_gb,
            vram_gb,
            vcpu,
            cloud_storage_gb,
            api_key,
            profile,
            runtime_mode,
            apply_trim,
        )?,
        Command::Plugins { command } => match command {
            PluginCommand::List { dir } => plugins::list_and_print(dir)?,
        },
        Command::Package { command } => match command {
            PackageCommand::Manifest { marketplace } => package::print_manifest(marketplace)?,
        },
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run(
    config_path: PathBuf,
    target: Option<String>,
    ram_gb: Option<f64>,
    vram_gb: Option<f64>,
    vcpu: Option<u16>,
    cloud_storage_gb: Option<f64>,
    api_key: Option<String>,
    profile: Option<ProfileArg>,
    runtime_mode: Option<RuntimeArg>,
    apply_trim: bool,
) -> Result<()> {
    let mut cfg = EngineConfig::load(config_path)?;
    if let Some(target) = target {
        cfg.target = target;
    }
    if let Some(ram_gb) = ram_gb {
        cfg.requirements.ram_gb = ram_gb;
    }
    if let Some(vram_gb) = vram_gb {
        cfg.requirements.vram_gb = vram_gb;
    }
    if let Some(vcpu) = vcpu {
        cfg.requirements.vcpu = vcpu;
    }
    if let Some(cloud_storage_gb) = cloud_storage_gb {
        cfg.requirements.cloud_storage_gb = cloud_storage_gb;
    }

    let report = hardware::inspect()?;
    println!("{}", report.render());

    let selected_profile = profile
        .map(TrimProfile::from)
        .unwrap_or_else(|| TrimProfile::from_name(&cfg.profile));
    let selected_runtime = if apply_trim {
        RuntimeMode::Production
    } else {
        runtime_mode.map(RuntimeMode::from).unwrap_or(cfg.runtime_mode)
    };
    let trim_plan = trim::plan(selected_profile, selected_runtime);
    trim::execute(&trim_plan)?;

    let bridge_plan = bridge::plan(&report, &cfg.requirements, &cfg.cloud, api_key.as_deref())?;
    println!("{}", bridge_plan.render());

    let manifests =
        plugins::load_manifests_with_policy(&PathBuf::from(&cfg.plugins_dir), &cfg.plugin_policy)?;
    plugins::print_hook_summary(&manifests, "pre_run");

    let launch = launcher::prepare(&cfg.target, &cfg.browser)?;
    println!("{}", launch.render());
    launcher::launch(&launch)?;

    plugins::print_hook_summary(&manifests, "post_run");
    Ok(())
}
