use std::io::Error;

use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};

pub struct MCP3008 {
    spidev: Spidev,
    tx_buf: [u8; 3],
}

impl MCP3008 {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let mut spidev = Spidev::open(path)?;

        let options = SpidevOptions::new()
            .max_speed_hz(1_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spidev.configure(&options)?;

        // 預設 channel 為 0
        let tx_buf = [0x01, 0x80, 0x00];

        Ok(MCP3008 { spidev, tx_buf })
    }

    pub fn set_channel(&mut self, channel: u8) {
        assert!(channel < 8, "Channel must be 0 to 7");
        self.tx_buf = [0x01, (8 + channel) << 4, 0x00];
    }

    pub fn read_adc(&self) -> Result<u16, Error> {
        let mut rx_buf = [0; 3];
        let mut transfer = SpidevTransfer::read_write(&self.tx_buf, &mut rx_buf);
        self.spidev.transfer(&mut transfer)?;

        // 組合並切割
        Ok(u16::from_be_bytes([rx_buf[1], rx_buf[2]]) & 0x03FF)
    }
}
