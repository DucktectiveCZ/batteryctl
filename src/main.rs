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

use clap::{Parser, ValueEnum};
mod battery;
mod daemon;

#[derive(ValueEnum, Clone, Debug)]
enum Operation {
  GetProperty,
  ListDevices,
  Daemon,
}

#[derive(Parser)]
struct Args {
  operation: Operation,
  #[arg(long, short, default_value("BAT0"))]
  device: Option<String>,
  #[arg(required_if_eq("operation", "get-property"))]
  property: Option<String>,
}

fn main() {
  let args = Args::parse();
  
  match args.operation {
    Operation::GetProperty => {
      match battery::get_device_property_raw(&args.device.unwrap(), &args.property.unwrap()) {
        Ok(val) => println!("{}", val),
        Err(e) => eprintln!("Error: {}", e.to_string()),
      }
    },
    Operation::ListDevices =>
      match battery::get_devices() {
        Ok(val) => val.iter().for_each(|device| {
          println!("{}", device);
        }),
        Err(e) => eprintln!("Error: {}", e),
      },
    Operation::Daemon =>
      match daemon::start() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
      }
  }
}
