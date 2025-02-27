// MIT License
// Copyright (c) 2024 DucktectiveCZ
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// provided to do so, subject to the following condition:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::{fmt, fs, path::PathBuf};

#[allow(dead_code)]
#[derive(Debug)]
pub enum BatteryError {
  IOError(String),
  ParseError(String),
}

impl fmt::Display for BatteryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BatteryError::IOError(e) => write!(f, "{}", e),
      BatteryError::ParseError(s) => write!(f, "{}", s),
    }
  }
}

impl std::error::Error for BatteryError { }

pub const BATTERIES_DIRECTORY: &str = "/sys/class/power_supply/";

#[allow(dead_code)]
pub fn get_device_property_raw(battery_name: &String, property_name: &str) -> Result<String, BatteryError> {
  let path: String = PathBuf::from(BATTERIES_DIRECTORY)
    .join(battery_name)
    .join(property_name)
    .to_string_lossy()
    .into_owned();
  match fs::read_to_string(path) {
    Ok(content) => Ok(content[..content.len() - 1].to_string()),
    Err(e) => Err(BatteryError::IOError(e.to_string())),
  }
}

#[allow(dead_code)]
pub fn get_devices() -> Result<Vec<String>, BatteryError> {
  let dirs: fs::ReadDir;
  match fs::read_dir(BATTERIES_DIRECTORY) {
    Ok(val) => dirs = val,
    Err(e) => return Err(BatteryError::IOError(e.to_string())),
  }
  let mut devices = Vec::new();
  for dir in dirs {
    let entry: fs::DirEntry;
    match dir {
      Ok(val) => entry = val,
      Err(e) => return Err(BatteryError::IOError(e.to_string())),
    }
    match entry.path().file_name() {
      Some(val) => devices.push(val
        .to_string_lossy()
        .to_string()),
      None => return Err(BatteryError::IOError("File name is None".to_string())),
    }
  }
  Ok(devices)
}

