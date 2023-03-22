use std::fs;

use super::SysfsObject;

pub struct BatteryPowerSupply {
    sysfs_obj: SysfsObject,
}

impl BatteryPowerSupply {
    fn new(sysfs_obj: SysfsObject) -> Self {
        Self { sysfs_obj }
    }

    pub fn refresh(&mut self) {
        self.sysfs_obj.refresh();
    }

    pub fn capacity(&self) -> u16 {
        self.sysfs_obj
            .value("POWER_SUPPLY_CAPACITY")
            .expect("Value POWER_SUPPLY_CAPACITY not present on battery!")
            .parse()
            .expect("POWER_SUPPLY_CAPACITY had unexpected format")
    }

    pub fn charging(&self) -> bool {
        self.sysfs_obj
            .value("POWER_SUPPLY_STATUS")
            .expect("Value POWER_SUPPLY_STATUS not present on battery!")
            == "Charging"
    }
}

const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";

pub fn get_battery() -> Option<BatteryPowerSupply> {
    let Ok(rd) = fs::read_dir(POWER_SUPPLY_PATH) else {
        return None;
    };

    for entry in rd {
        let Ok(entry) = entry else {
            continue;
        };
        let sysfs_obj = SysfsObject::new(entry.path());
        if sysfs_obj.value("POWER_SUPPLY_TYPE") == Some(&String::from("Battery")) {
            return Some(BatteryPowerSupply::new(sysfs_obj));
        }
    }

    None
}
