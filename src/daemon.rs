// MIT License
// Copyright (c) 2024 Pepa
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

use inotify::{EventMask, Inotify, WatchMask};
use std::{fmt, path::PathBuf};
use crate::battery;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DaemonError {
  IOError(String),
  AsyncError(String),
  INotifyError(String),
}

impl std::error::Error for DaemonError { }

impl fmt::Display for DaemonError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::IOError(e) => write!(f, "IO error: {}", e),
      Self::AsyncError(e) => write!(f, "Async error: {}", e),
      Self::INotifyError(e) => write!(f, "INotify error: {}", e),
    }
  }
}

const BATTERIES_DIRECTORY: &str = "/sys/class/power_supply/";

#[allow(dead_code)]
pub fn start() -> Result<(), DaemonError> {
  eprintln!("Feature not implemented");
  let mut inotify_instance: Inotify;
  match Inotify::init() {
    Ok(val) => inotify_instance = val,
    Err(e) => return Err(DaemonError::INotifyError(e.to_string())),
  }
  
  match add_watches(&inotify_instance) {
    Ok(_) => (),
    Err(e) => return Err(e),
  }

  loop {
    let _ = handle_events(&mut inotify_instance);
  }
}

fn watch_file<TCallback>(inotify_instance: &Inotify, file: String, callback: TCallback) -> Result<(), DaemonError>
where TCallback: Fn()
{
  Ok(())
}

fn handle_events(inotify_instance: &mut Inotify) -> Result<(), DaemonError> {
  let mut buffer = [0; 1024]; 

  let events = match inotify_instance.read_events_blocking(&mut buffer) {
    Ok(val) => val,
    Err(e) => return Err(DaemonError::INotifyError(e.to_string())),
  };

  for event in events {
    if event.mask.contains(EventMask::MODIFY) {
      if let Some(name) = event.name {
        match modified(&name.to_string_lossy().into_owned()) {
          Ok(_) => (),
          Err(e) => return Err(e),
        }
      } else {
        return Err(DaemonError::INotifyError("@event.name is None".to_string()));
      }
    }
  }
  Ok(())
}

fn modified(file: &String) -> Result<(), DaemonError> {
  Ok(())
}

fn add_watches(inotify_instance: &Inotify) -> Result<(), DaemonError> {
  let properties = vec![
    "capacity",
  ];
  let batteries = battery::get_devices()
    .into_iter()
    .filter(|s| s.starts_with());
  for property in properties {
    let mut path = PathBuf::from(BATTERIES_DIRECTORY);
    path.push(property);
    println!("Adding: {}", path.to_string_lossy().to_owned());
    match inotify_instance
      .watches()
      .add(path, WatchMask::MODIFY) {
      Ok(_) => (),
      Err(e) => return Err(DaemonError::INotifyError(e.to_string())),
    }
  }
  Ok(())
}

