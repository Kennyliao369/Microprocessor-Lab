use std::{error::Error, thread, time::Duration};

use lab02::Gpio;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Problem parsing config: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application problem: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // pin: 7 13 29 33
    let pins: [u32; 4] = [396, 255, 398, 389];

    match config {
        Config::Led(index, state) => {
            // 初始化
            let Some(pin) = pins.get(index) else {
                return Err("Invalid LED index".into());
            };
            let led = Gpio::new(*pin);
            led.export()?;
            led.set_direction("out")?;

            // 執行
            led.set_value(state)?;

            // 釋放
            if state == 0 {
                led.unexport()?;
            }
        }
        Config::Shine(count) => {
            // 初始化
            let mut leds = vec![];
            for pin in pins {
                let led = Gpio::new(pin);
                led.export()?;
                led.set_direction("out")?;
                leds.push(led);
            }

            // 執行
            let helf = leds.len() / 2;
            for i in 0..count {
                for (index, led) in leds.iter().enumerate() {
                    let offset = u32::from(index < helf);
                    led.set_value((i + offset) % 2)?;
                }
                thread::sleep(Duration::from_millis(500));
            }

            // 釋放
            for led in leds {
                led.unexport()?;
            }
        }
    }
    Ok(())
}

enum Config {
    Led(usize, u32),
    Shine(u32),
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args[1] == "Shine" {
            match args[2].parse::<u32>() {
                Ok(x) => Ok(Config::Shine(x)),
                Err(_) => Err("Shine count parse error"),
            }
        } else if args[1].starts_with("LED") {
            let Ok(index) = args[1].trim_start_matches("LED").parse::<usize>() else {
                return Err("Invalid LED number");
            };
            match args[2].as_str() {
                "on" => Ok(Config::Led(index, 1)),
                "off" => Ok(Config::Led(index, 0)),
                _ => Err("Invalid LED state"),
            }
            // let Ok(state) = args[2].parse::<u32>() else {
            //     return Err("Invalid LED state");
            // };
            // Ok(Config::Led(index, state))
        } else {
            Err("Unknown argument")
        }
    }
}
