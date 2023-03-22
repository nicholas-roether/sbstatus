use std::{thread, time::Duration};

use system::player::PlayerDataCollector;

mod system;

fn main() {
    let mut data_fetcher = PlayerDataCollector::new().unwrap();

    loop {
        data_fetcher.refresh();
        println!("{:?}", data_fetcher.player_data());
        thread::sleep(Duration::from_millis(200));
    }
}
