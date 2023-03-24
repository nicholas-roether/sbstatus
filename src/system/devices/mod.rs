use std::borrow::Cow;

pub mod power_supply;

pub struct DeviceScanner {
    enumerator: udev::Enumerator,
}

impl DeviceScanner {
    pub fn new(subsystem: &str) -> Self {
        let mut enumerator = udev::Enumerator::new().expect("Failed to create udev enumerator");
        enumerator
            .match_subsystem(subsystem)
            .expect("Failed to apply udev enumerator subsystem filter");
        Self { enumerator }
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

    pub fn device_type(&self) -> Cow<str> {
        dbg!(&self.device);
        self.device
            .devtype()
            .expect("udev device doesn't specify a type")
            .to_string_lossy()
    }

    pub fn device_name(&self) -> Cow<str> {
        self.device
            .devnode()
            .expect("udev device has no path associated")
            .to_string_lossy()
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
