use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeMode {
    Production,
    DryRun,
    StressTest,
    Debug,
}

impl RuntimeMode {
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "production" | "prod" => Self::Production,
            "stress-test" | "stress" => Self::StressTest,
            "debug" => Self::Debug,
            _ => Self::DryRun,
        }
    }

    pub fn applies_real_changes(self) -> bool {
        matches!(self, Self::Production)
    }

    pub fn allows_simulation(self) -> bool {
        matches!(self, Self::StressTest | Self::Debug)
    }
}
