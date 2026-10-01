use std::{error::Error, fs, io::Write};

pub struct Gpio {
    pin: u32,
}

impl Gpio {
    pub fn new(pin: u32) -> Gpio {
        Gpio { pin }
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let gpio_dir = format!("/sys/class/gpio/gpio{}", self.pin);
        if !fs::exists(&gpio_dir)? {
            let path = "/sys/class/gpio/export";
            let mut file = fs::OpenOptions::new().write(true).open(path)?;
            file.write_all(self.pin.to_string().as_bytes())?;
        }

        Ok(())
    }

    pub fn set_direction(&self, status: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("/sys/class/gpio/gpio{}/direction", self.pin);
        match status {
            "in" | "out" => {
                let mut file = fs::OpenOptions::new().write(true).open(path)?;
                file.write_all(status.as_bytes())?;
            }
            _ => return Err(format!("Invalid direction value: {status}").into()),
        }
        Ok(())
    }

    pub fn set_value(&self, value: u32) -> Result<(), Box<dyn Error>> {
        let path = format!("/sys/class/gpio/gpio{}/value", self.pin);
        match value {
            0 | 1 => {
                let mut file = fs::OpenOptions::new().write(true).open(path)?;
                file.write_all(value.to_string().as_bytes())?;
            }
            _ => return Err(format!("Invalid value: {value}").into()),
        }
        Ok(())
    }

    pub fn unexport(&self) -> Result<(), Box<dyn Error>> {
        let path = "/sys/class/gpio/unexport";
        let mut file = fs::OpenOptions::new().write(true).open(path)?;
        file.write_all(self.pin.to_string().as_bytes())?;
        Ok(())
    }
}

// /sys/class/gpio/
// echo 255 > export

// /sys/class/gpio/gpoi255
// echo out > direction
// echo 1 > value

// /sys/class/gpio/
// echo 255 > unexport

// 丟棄時自動卸載容易有問題
// impl Drop for Gpio {
//     fn drop(&mut self) {
//         let path = "/sys/class/gpio/unexport";
//         match fs::OpenOptions::new().write(true).open(path) {
//             Ok(mut file) => {
//                 let _ = file.write_all(self.pin.to_string().as_bytes());
//             }
//             Err(e) => eprintln!("Fail to unexport: {e}"),
//         };
//     }
// }
