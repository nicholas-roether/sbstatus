use std::{thread, time::Duration};

use system::player::PlayerDataCollector;

mod system;

fn main() {
    let battery = system::sysfs::battery::get_battery().unwrap();
    println!("{}", battery.capacity())
}
