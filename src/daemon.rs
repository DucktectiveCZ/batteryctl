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

use std::{fmt, process::Command, thread::sleep, time::Duration};

use crate::battery;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DaemonError {
    IO(String),
    Async(String),
    INotify(String),
}

impl std::error::Error for DaemonError { }

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO error: {}", e),
            Self::Async(e) => write!(f, "Async error: {}", e),
            Self::INotify(e) => write!(f, "INotify error: {}", e),
        }
    }
}

pub struct DaemonConfig {
    pub battery: String,
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

impl std::fmt::Display for DaemonConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "battery: {}, read_delay: {}, good_capacity: {}, okay_capacity: {}, bad_capacity: {}, critical_capacity: {}",
            self.battery,
            self.read_delay_ms,
            self.good_capacity,
            self.okay_capacity,
            self.bad_capacity,
            self.critical_capacity )
    }
}

struct Status {
    pub good_handled: bool,
    pub okay_handled: bool,
    pub bad_handled: bool,
    pub critical_handled: bool,
}

impl Status {
    fn new() -> Status {
        Self {
            good_handled: false,
            okay_handled: false,
            bad_handled: false,
            critical_handled: false,
        }
    }
}

#[allow(dead_code)]
pub fn start(config: &DaemonConfig) -> Result<(), DaemonError> {
    println!("[WARN] Feature not fully implemented yet!");
    println!("[INFO] Starting batteryctl daemon with config {}", config);

    let mut status = Status::new();

    let mut capacity_buf: u8;

    loop {
        match battery::get_device_property_raw(&config.battery, "capacity") {
            Ok(val) => capacity_buf = val.parse::<u8>().unwrap(),
            Err(e) => return Err(DaemonError::IO(e.to_string())),
        }

        match handle_capacity(config, &capacity_buf, &mut status) {
            Ok(_) => println!("[INFO] Capacity handled."),
            Err(e) => return Err(e),
        }

        sleep(Duration::from_millis(config.read_delay_ms));
    }
}

fn handle_capacity(config: &DaemonConfig, capacity: &u8, status: &mut Status) -> Result<(), DaemonError> {
    if !status.critical_handled && *capacity <= config.critical_capacity {
        handle_critical(config);
    }
    else if !status.bad_handled && *capacity <= config.bad_capacity {
        handle_bad(config);
    }
    else if !status.okay_handled && *capacity <= config.okay_capacity {
        handle_okay(config);
    }
    else if !status.good_handled && *capacity >= config.good_capacity {
        handle_good(config);
    } else {
        handle_other();
    }

    Ok(())
}

fn handle_good(config: &DaemonConfig) {
    println!("[INFO] Good capacity");

    if let Some(handler) = &config.good_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(handler)
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Good capacity handler failed: {}", e),
        }
    }
}
fn handle_okay(config: &DaemonConfig) {
    println!("[INFO] Okay capacity");

    if let Some(handler) = &config.okay_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(handler)
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Okay capacity handler failed: {}", e),
        }
    }
}
fn handle_bad(config: &DaemonConfig) {
    println!("[INFO] Bad capacity");

    if let Some(handler) = &config.bad_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(handler)
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Bad capacity handler failed: {}", e),
        }
    }
}
fn handle_critical(config: &DaemonConfig) {
    println!("[INFO] Critical capacity");

    if let Some(handler) = &config.good_capacity_handler {
        let status = Command::new("bash")
            .arg("-c")
            .arg(handler)
            .status();

        match status {
            Ok(_) => (),
            Err(e) => eprintln!("[ERROR] Critical capacity handler failed: {}", e),
        }
    }
}
fn handle_other() {
    println!("[WARN] Unknown capacity");
}

