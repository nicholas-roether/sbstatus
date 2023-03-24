use std::borrow::Cow;

use super::{Device, DeviceScanner};

pub struct PowerSupplyScanner {
    device_scanner: DeviceScanner,
}

impl PowerSupplyScanner {
    pub fn new() -> Self {
        Self {
            device_scanner: DeviceScanner::new("power_source"),
        }
    }

    pub fn find_battery(&mut self, name: Option<&str>) -> Option<Battery> {
        for device in self.device_scanner.get_devices() {
            if device.device_type() != "Battery" {
                continue;
            }
            if let Some(name) = name {
                if device.device_name() != name {
                    continue;
                }
            }
            return Some(Battery::new(device));
        }
        None
    }
}

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
