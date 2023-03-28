use std::borrow::Cow;

use anyhow::{Context, Result};

pub mod power_supply;

pub struct DeviceScanner {
    enumerator: udev::Enumerator,
}

impl DeviceScanner {
    pub fn new() -> Result<Self> {
        let mut enumerator = udev::Enumerator::new().context("Failed to create udev enumerator")?;
        Ok(Self { enumerator })
    }

    pub fn filter_subsystem(&mut self, subsystem: &str) -> Result<()> {
        self.enumerator
            .match_subsystem(subsystem)
            .context("Invalid subsystem provided")
    }

    pub fn get_devices(&mut self) -> Vec<Device> {
        self.enumerator
            .scan_devices()
            .expect("Failed to scan for udev devices")
            .map(Into::into)
            .collect()
    }
}

#[derive(Debug)]
pub struct Device {
    device: udev::Device,
}

impl Device {
    fn new(device: udev::Device) -> Self {
        Self { device }
    }

    pub fn attr(&self, name: &str) -> Option<Cow<str>> {
        let Some(val_osstr) = self.device.attribute_value(name) else {
            return None;
        };
        Some(val_osstr.to_string_lossy())
    }
}

impl From<udev::Device> for Device {
    fn from(value: udev::Device) -> Self {
        Self::new(value)
    }
}
