use serde::Deserialize;

use crate::modules::battery::BatteryConfig;

#[derive(Debug, Deserialize)]
struct Modules {
    battery: BatteryConfig,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    show: Vec<String>,
    modules: Modules,
}
