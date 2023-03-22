use std::{ffi::OsStr, path::Path};

pub mod power_supply;

#[macro_export]
macro_rules! attribute_getter {
    ($name:ident, $ty:ty) => {
        pub fn $name(&self) -> Option<$ty> {
            let Some(os_str) = $crate::system::devices::DeviceWrapper::device(self).attribute_value(stringify!($ident)) else {
                                                                                        return None;
                                                                                    };
            os_str.to_string_lossy().parse().ok()
        }
    };
}

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

    pub fn get_devices(&mut self) -> Vec<udev::Device> {
        self.enumerator
            .scan_devices()
            .expect("Failed to scan for udev devices")
            .collect()
    }
}

pub trait DeviceWrapper {
    fn device(&self) -> &udev::Device;

    fn name(&self) -> String {
        self.device()
            .devnode()
            .map(Path::as_os_str)
            .map(OsStr::to_string_lossy)
            .expect("Unexpected unnamed device")
            .into_owned()
    }
}
