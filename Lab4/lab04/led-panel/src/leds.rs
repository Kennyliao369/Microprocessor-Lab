use std::{
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::Gpio;

pub struct Leds(Vec<Gpio>);

impl Leds {
    pub fn new(v: &[u32]) -> Self {
        let gpios: Vec<Gpio> = v.iter().map(|&pin| Gpio::new(pin)).collect();
        Leds(gpios)
    }

    pub fn exports(&self) -> Result<(), Box<dyn Error>> {
        let mut any_error = Ok(());
        for gpio in self.0.iter() {
            if let Err(e) = gpio.export() {
                any_error = Err(e);
            }
        }
        any_error
    }

    pub fn set_direction(&self, state: &str) -> Result<(), Box<dyn Error>> {
        let mut any_error = Ok(());
        for gpio in self.0.iter() {
            if let Err(e) = gpio.set_direction(state) {
                any_error = Err(e);
            }
        }
        any_error
    }

    pub fn shine(&self, index: u32, state: bool) -> Result<(), Box<dyn Error>> {
        let gpio = self.0.get(index as usize).ok_or("LED index out of range")?;
        gpio.set_value(u32::from(state))
    }

    pub fn all_off(&self) -> Result<(), Box<dyn Error>> {
        let mut any_error = Ok(());
        for gpio in self.0.iter() {
            if let Err(e) = gpio.set_value(0) {
                any_error = Err(e);
            }
        }
        any_error
    }

    pub async fn switch(&self, times: u32) {
        let half = self.0.len() / 2;
        for round in 0..times {
            for (index, led) in self.0.iter().enumerate() {
                let offset = (index < half) as u32;
                let _ = led.set_value((round + offset) % 2);
            }
            trpl::sleep(Duration::from_millis(500)).await;
        }
        let _ = self.all_off();
    }

    pub async fn running(&self, reverse: &AtomicBool) {
        let length = self.0.len();
        let mut index = 0;
        loop {
            let _ = self.0[index].set_value(0);
            index = if reverse.load(Ordering::Relaxed) {
                (index + length - 1) % length
            } else {
                (index + 1) % length
            };
            let _ = self.0[index].set_value(1);
            trpl::sleep(Duration::from_millis(500)).await;
        }
    }
}
