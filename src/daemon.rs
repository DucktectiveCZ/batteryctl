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

use std::{collections::HashMap, fmt, process::Command, thread::sleep, time::Duration};

use crate::battery;
use crate::config;

#[derive(Debug)]
pub enum DaemonError {
    IO(String),
}

impl std::error::Error for DaemonError { }

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

struct BatteryStatus {
    pub good_handled: bool,
    pub okay_handled: bool,
    pub bad_handled: bool,
    pub critical_handled: bool,
}

struct DaemonStatus {
    pub batteries: HashMap<String, BatteryStatus>,
}

impl DaemonStatus {
    pub fn new() -> DaemonStatus {
        Self {
            batteries: HashMap::new()
        }
    }
}

pub fn start(config: &config::Config) -> Result<(), DaemonError> {
    println!("[WARN] Beta feature");
    // println!("[INFO] Starting batteryctl daemon with config {:?}", config);

    let mut status = DaemonStatus::new();

    let mut capacity_buf: u8;

    loop {
        for bat in &config.batteries {
            match battery::get_device_property_raw(bat, "capacity") {
                Ok(val) => capacity_buf = val.parse::<u8>().unwrap(),
                Err(e) => return Err(DaemonError::IO(e.to_string())),
            }

            match handle_capacity(config, &capacity_buf, &mut status, bat) {
                Ok(_) => println!("[INFO] Capacity handled for {bat}."),
                Err(e) => return Err(e),
            }
        }

        sleep(Duration::from_millis(config.read_delay_ms));
    }
}

// TODO: Set the handled variables
fn handle_capacity(config: &config::Config, capacity: &u8, status: &mut DaemonStatus, bat: &String) -> Result<(), DaemonError> {
    if !status.batteries[bat].critical_handled && *capacity <= config.critical_capacity {
        handle_critical(config, bat);
    }
    else if !status.batteries[bat].bad_handled && *capacity <= config.bad_capacity {
        handle_bad(config, bat);
    }
    else if !status.batteries[bat].okay_handled && *capacity <= config.okay_capacity {
        handle_okay(config, bat);
    }
    else if !status.batteries[bat].good_handled {
        handle_good(config, bat);
    }

    Ok(())
}

fn handle_good(config: &config::Config, bat: &String) {
    println!("[INFO] Good capacity of {bat}");

    if let Some(handler) = &config.good_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(format!("{} {}", handler, bat))
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Good capacity handler failed: {e}"),
        }
    }
}
fn handle_okay(config: &config::Config, bat: &String) {
    println!("[INFO] Okay capacity");

    if let Some(handler) = &config.okay_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(format!("{} {}", handler, bat))
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Okay capacity handler failed: {e}"),
        }
    }
}
fn handle_bad(config: &config::Config, bat: &String) {
    println!("[INFO] Bad capacity");

    if let Some(handler) = &config.bad_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(format!("{} {}", handler, bat))
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Bad capacity handler failed: {e}"),
        }
    }
}
fn handle_critical(config: &config::Config, bat: &String) {
    println!("[INFO] Critical capacity");

    if let Some(handler) = &config.critical_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(format!("{} {}", handler, bat))
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Critical capacity handler failed: {e}"),
        }
    }
}

