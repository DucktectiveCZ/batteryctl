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

mod battery;
mod daemon;
mod config;
mod args;
mod presets;

use args::{Args, Operation};
use clap::Parser;
use config::Config;

const VERSION: [u8; 3] = [1, 1, 0];

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut config = config::Config::load().unwrap();

    match args.operation {
        Operation::GetProperty => operation_get_property(&args),
        Operation::ListDevices => operation_list_devices(),
        Operation::Daemon      => operation_daemon(&config),
        Operation::ConfigGet   => operation_config_get(&args, &config),
        Operation::ConfigSet   => operation_config_set(&args, &mut config),
        Operation::Version |
        Operation::V           => operation_version(),
    }
}

fn operation_get_property(args: &Args) -> Result<(), String> {
    match battery::get_device_property_raw(
        args.device.as_ref().unwrap(),
        args.property.as_ref().unwrap()
    ) {
        Ok(val) => println!("{}", val),
        Err(e) => return Err(e.to_string()),
    };

    Ok(())
}
fn operation_list_devices() -> Result<(), String> {
    match battery::get_devices() {
        Ok(val) =>
            val
                .iter()
                .for_each(|device| println!("{}", device) ),
        Err(e) => return Err(e.to_string()),
    };

    Ok(())
}
fn operation_daemon(config: &Config) -> Result<(), String> {
    match daemon::start(config) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
fn operation_config_get(args: &Args, config: &Config) -> Result<(), String> {
    match args.key.as_ref().unwrap() {
        args::ConfigKey::Batteries => println!("{:?}", config.batteries),
        args::ConfigKey::ReadDelay => println!("{}", config.read_delay_ms),
        args::ConfigKey::GoodCapacity => println!("{}", config.good_capacity),
        args::ConfigKey::OkayCapacity => println!("{}", config.okay_capacity),
        args::ConfigKey::BadCapacity => println!("{}", config.bad_capacity),
        args::ConfigKey::CriticalCapacity => println!("{}", config.critical_capacity),
        args::ConfigKey::GoodCapacityHandler => println!("{:?}", config.good_capacity_handler),
        args::ConfigKey::OkayCapacityHandler => println!("{:?}", config.okay_capacity_handler),
        args::ConfigKey::BadCapacityHandler => println!("{:?}", config.bad_capacity_handler),
        args::ConfigKey::CriticalCapacityHandler => println!("{:?}", config.critical_capacity_handler),
    }

    Ok(())
}
fn operation_config_set(args: &Args, config: &mut Config) -> Result<(), String> {
    match args.key.as_ref().unwrap() {
        args::ConfigKey::Batteries => eprintln!("[Error] Setting array properties via the CLI isn't supported yet. Edit the configuration file at '~/.config/batteryctl/ instead.'"),
        args::ConfigKey::ReadDelay => config.read_delay_ms = args.value.clone()
            .unwrap()
            .parse::<u64>()
            .expect("[Error] The value has to be a valid unsigned 64-bit integer."),
        args::ConfigKey::GoodCapacity => config.good_capacity = args.value.clone()
            .unwrap()
            .parse::<u8>()
            .expect("[Error] The value has to be a valid unsigned 8-bit integer."),
        args::ConfigKey::OkayCapacity => config.okay_capacity = args.value.clone()
            .unwrap()
            .parse::<u8>()
            .expect("[Error] The value has to be a valid unsigned 8-bit integer."),
        args::ConfigKey::BadCapacity => config.bad_capacity = args.value.clone()
            .unwrap()
            .parse::<u8>()
            .expect("[Error] The value has to be a valid unsigned 8-bit integer."),
        args::ConfigKey::CriticalCapacity => config.critical_capacity = args.value.clone()
            .unwrap()
            .parse::<u8>()
            .expect("[Error] The value has to be a valid unsigned 8-bit integer."),
        args::ConfigKey::GoodCapacityHandler =>
            config.good_capacity_handler = Some(args.value.clone().unwrap()),
        args::ConfigKey::OkayCapacityHandler =>
            config.okay_capacity_handler = Some(args.value.clone().unwrap()),
        args::ConfigKey::BadCapacityHandler =>
            config.bad_capacity_handler = Some(args.value.clone().unwrap()),
        args::ConfigKey::CriticalCapacityHandler =>
            config.critical_capacity_handler = Some(args.value.clone().unwrap()),
    };

    config.save();

    Ok(())
}
fn operation_version() -> Result<(), String> {
    println!(
        "batteryctl v{}.{}.{}",
        VERSION[0],
        VERSION[1],
        VERSION[2],
    );

    Ok(())
}
