// spidev 官方範例
// https://github.com/rust-embedded/rust-spidev/blob/master/examples/spidev-bidir.rs

use std::{thread, time::Duration};

use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};

fn main() {
    let mut spidev = Spidev::open("/dev/spidev3.0").unwrap();

    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spidev.configure(&options).unwrap();

    // must be 0..=7
    let channel: u8 = 0;

    let tx_buf = [0x01, (8 + channel) << 4, 0x00];
    let mut rx_buf = [0; 3];

    loop {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spidev.transfer(&mut transfer).unwrap();

        // 組合並切割
        let raw = u16::from_be_bytes([rx_buf[1], rx_buf[2]]) & 0x03FF;
        let volt = f32::from(raw) / 1024.0 * 3.3;
        println!("row: {raw}; volt: {volt}");

        thread::sleep(Duration::from_millis(100));
    }
}
