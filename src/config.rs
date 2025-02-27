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

use std::str::FromStr;
use std::{fs, path::PathBuf};
use std::env;
use serde::{Serialize, Deserialize};
use crate::presets;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub batteries: Vec<String>,
    pub read_delay_ms: u64,

    pub good_capacity: u8,
    pub okay_capacity: u8,
    pub bad_capacity: u8,
    pub critical_capacity: u8,

    pub good_capacity_handler: Option<String>,
    pub okay_capacity_handler: Option<String>,
    pub bad_capacity_handler: Option<String>,
    pub critical_capacity_handler: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            batteries: Vec::new(),
            read_delay_ms: 180000,
            good_capacity: 75,
            okay_capacity: 50,
            bad_capacity: 20,
            critical_capacity: 5,
            good_capacity_handler: None,
            okay_capacity_handler: None,
            bad_capacity_handler: None,
            critical_capacity_handler: None,
        }
    }
    pub fn save(&self) {
        let contents = toml::to_string_pretty(self).unwrap();

        let path = get_config_path().unwrap();

        fs::write(path, contents).unwrap();
    }

    pub fn load() -> std::io::Result<Self> {
        let path = get_config_path()?;

        let contents;
        match fs::read_to_string(&path) {
            Ok(val) => contents = val,
            Err(_) => {
                fs::write(&path, presets::CONFIG)?;
                return Ok(Self::new())
            }
        }

        match toml::de::from_str::<Self>(&contents) {
            Ok(val) => Ok(val),
            Err(_) => {
                fs::write(&path, presets::CONFIG)?;
                return Ok(Self::new())
            }
        }
    }
}

fn get_config_path() -> std::io::Result<PathBuf> {
    let xdg_home = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME")
        .map(|home| format!("{}/.config", home)))
        .unwrap();

    let mut path = PathBuf::from_str(xdg_home.as_str()).unwrap();

    path.push("batteryctl");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("config.toml");

    Ok(path)
}

