use serde::Deserialize;

use super::{ModuleConfig, ModuleParams};

#[derive(Debug, Deserialize)]
pub struct BatteryConfig {
    device_name: Option<String>,
    warn_cutoff: Option<u32>,
    critical_cutoff: Option<u32>,
    charging_indicator: Option<String>,
    show_charging_indicator: Option<bool>,
    #[serde(flatten)]
    general: ModuleConfig,
}

#[derive(Debug)]
pub struct BatteryParams {
    device_name: Option<String>,
    warn_cutoff: u32,
    critical_cutoff: u32,
    charging_indicator: String,
    show_charging_indicator: bool,
    general: ModuleParams,
}

impl From<BatteryConfig> for BatteryParams {
    fn from(cfg: BatteryConfig) -> Self {
        BatteryParams {
            device_name: cfg.device_name,
            warn_cutoff: cfg.warn_cutoff.unwrap_or(20),
            critical_cutoff: cfg.critical_cutoff.unwrap_or(10),
            charging_indicator: cfg.charging_indicator.unwrap_or(String::from("🗲")),
            show_charging_indicator: cfg.show_charging_indicator.unwrap_or(true),
            general: ModuleParams {
                brackets: cfg.general.brackets.unwrap_or(true),
                label: cfg.general.label.unwrap_or(String::from("Bat")),
                show_label: cfg.general.show_label.unwrap_or(true),
            },
        }
    }
}
