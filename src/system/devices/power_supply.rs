use crate::attribute_getter;

use super::{DeviceScanner, DeviceWrapper};

pub struct PowerSupplyScanner {
    device_scanner: DeviceScanner,
}

impl PowerSupplyScanner {
    pub fn new() -> Self {
        Self {
            device_scanner: DeviceScanner::new("powersource"),
        }
    }

    pub fn get_first_battery(&mut self) -> Option<Battery> {
        self.iter_bateries().next()
    }

    pub fn get_battery(&mut self, name: &str) -> Option<Battery> {
        self.iter_bateries().find(|bat| bat.name() == name)
    }

    fn iter_bateries(&mut self) -> impl Iterator<Item = Battery> + '_ {
        self.device_scanner
            .get_devices()
            .into_iter()
            .filter_map(|dev| {
                let Some(device_type) = dev.attribute_value("type") else {
                    return None;
                };
                (device_type.to_string_lossy() == "Battery").then(|| Battery::new(dev))
            })
    }
}

#[derive(Debug)]
pub struct Battery {
    device: udev::Device,
}

impl Battery {
    fn new(device: udev::Device) -> Self {
        Self { device }
    }

    attribute_getter!(capacity, u32);

    attribute_getter!(status, String);

    pub fn is_charging(&self) -> Option<bool> {
        self.status().map(|status| status == *"Charging")
    }
}

impl DeviceWrapper for Battery {
    fn device(&self) -> &udev::Device {
        &self.device
    }
}
