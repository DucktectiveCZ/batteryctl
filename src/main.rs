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

use clap::{Parser, ValueEnum};
mod battery;

#[derive(ValueEnum, Clone, Debug)]
enum Operation {
  GetProperty,
}

#[derive(Parser)]
struct Args {
  operation: Operation,
  device: String,
  args: String,
}

fn main() {
  let args = Args::parse();
  
  match args.operation {
    Operation::GetProperty => {
      match battery::get_battery_property_raw(&args.device, &args.args) {
        Ok(val) => println!("{}", val),
        Err(e) => eprintln!("Error: {}", e.to_string()),
      }
    },
  }
}