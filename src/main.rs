use std::{thread, time::Duration};

use system::player::PlayerDataCollector;

use crate::system::devices::power_supply::PowerSupplyScanner;

mod system;

mod modules;

mod config;

fn main() {
    let mut psupp_scanner = PowerSupplyScanner::new();
    let battery = psupp_scanner.get_first_battery();

    println!("{:?}", battery)
}
