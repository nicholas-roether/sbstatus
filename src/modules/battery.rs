use std::rc::Rc;

use serde::Deserialize;

use crate::system::devices::power_supply::{self, PowerSupplies};

use super::{ModuleConfig, ModuleParams};

#[derive(Debug, Deserialize)]
pub struct BatteryConfig {
    warn_cutoff: Option<u32>,
    critical_cutoff: Option<u32>,
    charging_indicator: Option<String>,
    show_charging_indicator: Option<bool>,
    #[serde(flatten)]
    general: ModuleConfig,
}

#[derive(Debug)]
struct BatteryParams {
    warn_cutoff: u32,
    critical_cutoff: u32,
    charging_indicator: String,
    show_charging_indicator: bool,
    general: ModuleParams,
}

impl From<BatteryConfig> for BatteryParams {
    fn from(cfg: BatteryConfig) -> Self {
        BatteryParams {
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

#[derive(Debug)]
pub struct BatteryModule {
    power_supplies: Rc<PowerSupplies>,
    params: BatteryParams,
}

impl BatteryModule {
    pub fn new(config: BatteryConfig, power_supplies: Rc<PowerSupplies>) -> Self {
        Self {
            params: config.into(),
            power_supplies,
        }
    }
}
