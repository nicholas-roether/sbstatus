use crate::system::devices::power_supply::get_power_supplies;

mod system;

mod modules;

mod config;

fn main() {
    let psupps = get_power_supplies();

    println!("{:?}", psupps)
}
