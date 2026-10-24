use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Write,
};

use Direction::{Input, Output};

#[derive(Clone, Copy)]
pub enum Direction {
    Output,
    Input,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Input => "in",
            Output => "out",
        };
        write!(f, "{text}")
    }
}

pub struct Gpio {
    pin: u32,
    value_file: File,
}

impl Gpio {
    pub fn new(pin: u32, direction: Direction) -> Result<Gpio, Box<dyn Error>> {
        export(pin)?;

        let value_dir = format!("/sys/class/gpio/gpio{}/value", pin);
        let value_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(value_dir)?;

        let gpio = Gpio { pin, value_file };
        gpio.set_direction(direction)?;

        Ok(gpio)
    }

    pub fn set_direction(&self, direction: Direction) -> Result<(), Box<dyn Error>> {
        let path = format!("/sys/class/gpio/gpio{}/direction", self.pin);
        let mut file = fs::OpenOptions::new().write(true).open(path)?;
        file.write_all(direction.to_string().as_bytes())?;
        Ok(())
    }

    pub fn set_value(&self, value: u32) -> Result<(), Box<dyn Error>> {
        match value {
            0 | 1 => {
                (&self.value_file).write_all(value.to_string().as_bytes())?;
            }
            _ => return Err(format!("Invalid value: {value}").into()),
        }
        Ok(())
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        let path = "/sys/class/gpio/unexport";
        match fs::OpenOptions::new().write(true).open(path) {
            Ok(mut file) => {
                let _ = file.write_all(self.pin.to_string().as_bytes());
            }
            Err(e) => eprintln!("Fail to unexport: {e}"),
        };
    }
}

// Tools
fn export(pin: u32) -> Result<(), Box<dyn Error>> {
    let gpio_dir = format!("/sys/class/gpio/gpio{}", pin);
    if !fs::exists(&gpio_dir)? {
        let path = "/sys/class/gpio/export";
        let mut file = fs::OpenOptions::new().write(true).open(path)?;
        file.write_all(pin.to_string().as_bytes())?;
    }
    Ok(())
}

// /sys/class/gpio/
// echo 255 > export

// /sys/class/gpio/gpoi255
// echo out > direction
// echo 1 > value

// /sys/class/gpio/
// echo 255 > unexport
