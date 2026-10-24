// spidev 官方範例
// https://github.com/rust-embedded/rust-spidev/blob/master/examples/spidev-bidir.rs

use std::{thread, time::Duration};

mod mcp3008;
use mcp3008::MCP3008;

mod gpio;
use gpio::Direction::Output;
use gpio::Gpio;

fn main() {
    let gpio1 = Gpio::new(396, Output).unwrap();
    let gpio2 = Gpio::new(255, Output).unwrap();

    let mut mcp3008 = MCP3008::new("/dev/spidev3.0").unwrap();
    mcp3008.set_channel(0);

    let mut is_on = false;
    loop {
        let raw = mcp3008.read_adc().unwrap();
        let volt = f32::from(raw) / 1024.0 * 3.3;
        println!("raw: {raw}; volt: {volt}");

        let should_on = raw <= 512;
        if should_on != is_on {
            let v = u32::from(should_on);
            gpio1.set_value(v).unwrap();
            gpio2.set_value(v).unwrap();
            is_on = should_on;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
