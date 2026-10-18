#[derive(PartialEq)]
pub enum LedMode {
    Shine { index: u32, power: bool },
    Switch { times: u32 },
    Running,
    Stop,
}

impl LedMode {
    pub fn from_uri(uri: &str) -> Result<LedMode, String> {
        let (path, query) = uri.split_once('?').unwrap_or((uri, ""));

        match path {
            // /index?LED=1&POWER=on
            "/shine" => {
                let mut index = None;
                let mut power = None;

                for pair in query.split("&") {
                    match pair.split_once("=") {
                        Some(("LED", v)) => {
                            index = Some(v.parse::<u32>().map_err(|_| "Unknown led index")?);
                        }
                        Some(("POWER", "ON")) => power = Some(true),
                        Some(("POWER", "OFF")) => power = Some(false),
                        _ => {}
                    }
                }

                let index = index.ok_or("Need led index")?;
                let power = power.ok_or("Need power status")?;
                Ok(LedMode::Shine { index, power })
            }
            // /switch?times=5
            "/switch" => {
                let times = query
                    .strip_prefix("times=")
                    .ok_or("Need times argument")?
                    .parse::<u32>()
                    .map_err(|_| "Times need to be number")?;
                Ok(LedMode::Switch { times })
            }
            "/running" => Ok(LedMode::Running),
            "/stop" => Ok(LedMode::Stop),
            _ => Err("Unknown path")?,
        }
    }
}
