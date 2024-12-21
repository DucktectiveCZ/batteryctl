use std::{fs, fmt, path::PathBuf};

#[allow(dead_code)]
#[derive(Debug)]
pub enum BatteryError {
  IOError(std::io::Error),
  ParseError(String),
}

impl fmt::Display for BatteryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BatteryError::IOError(e) => write!(f, "{}", e.to_string()),
      BatteryError::ParseError(s) => write!(f, "{}", s),
    }
  }
}

impl std::error::Error for BatteryError { }

const BATTERIES_DIRECTORY: &str = "/sys/class/power_supply/";

#[allow(dead_code)]
pub fn get_battery_property_raw(battery_name: &String, property_name: &str) -> Result<String, BatteryError> {
  let path: String = PathBuf::from(BATTERIES_DIRECTORY)
    .join(battery_name)
    .join(property_name)
    .to_string_lossy()
    .into_owned();
  match fs::read_to_string(path) {
    Ok(content) => Ok(content),
    Err(e) => Err(BatteryError::IOError(e)),
  }
}

