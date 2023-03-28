use std::borrow::Cow;

use anyhow::Result;

use super::{Device, DeviceScanner};

#[derive(Debug)]
pub struct Battery {
    device: Device,
}

impl Battery {
    fn new(device: Device) -> Self {
        Self { device }
    }

    pub fn capacity(&self) -> u8 {
        self.device
            .attr("capacity")
            .expect("Battery doesn't specify capacity")
            .parse()
            .expect("Battery capacity had unexpected value")
    }
}

#[derive(Debug)]
pub struct Mains {
    device: Device,
}

impl Mains {
    fn new(device: Device) -> Self {
        Self { device }
    }

    pub fn is_online(&self) -> bool {
        self.device.attr("online") == Some(Cow::Borrowed("1"))
    }
}

#[derive(Debug, Default)]
pub struct PowerSupplies {
    pub battery: Option<Battery>,
    pub mains: Option<Mains>,
}

pub fn get_power_supplies() -> Result<PowerSupplies> {
    let mut psupps = PowerSupplies::default();
    let mut dev_scanner = DeviceScanner::new()?;
    dev_scanner.filter_subsystem("power_supply")?;

    for device in dev_scanner.get_devices() {
        match device.attr("type").as_deref() {
            Some("Battery") => {
                psupps.battery.get_or_insert(Battery::new(device));
            }
            Some("Mains") => {
                psupps.mains.get_or_insert(Mains::new(device));
            }
            _ => (),
        }
    }

    Ok(psupps)
}
