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

#[derive(ValueEnum, Clone, Debug)]
pub enum Operation {
    GetProperty,
    ListDevices,
    Daemon,
    ConfigGet,
    ConfigSet,
    Version,
    V,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConfigKey {
    Batteries,
    ReadDelay,
    GoodCapacity,
    OkayCapacity,
    BadCapacity,
    CriticalCapacity,
    GoodCapacityHandler,
    OkayCapacityHandler,
    BadCapacityHandler,
    CriticalCapacityHandler,
}

#[derive(Parser)]
pub struct Args {
    pub operation: Operation,
    #[arg(required_if_eq("operation", "config-get"), required_if_eq("operation", "config-set"))]
    pub key: Option<ConfigKey>,
    #[arg(required_if_eq("operation", "config-set"))]
    pub value: Option<String>,
    #[arg(long, short, default_value("BAT0"))]
    pub device: Option<String>,
    #[arg(required_if_eq("operation", "get-property"))]
    pub property: Option<String>,
    #[arg(long, short)]
    pub read_delay: Option<u64>,
    #[arg(long, short)]
    pub good_capacity: Option<u8>,
    #[arg(long, short)]
    pub okay_capacity: Option<u8>,
    #[arg(long, short)]
    pub bad_capacity: Option<u8>,
    #[arg(long, short)]
    pub critical_capacity: Option<u8>,
    #[arg(long)]
    pub good_capacity_handler: Option<String>,
    #[arg(long)]
    pub okay_capacity_handler: Option<String>,
    #[arg(long)]
    pub bad_capacity_handler: Option<String>,
    #[arg(long)]
    pub critical_capacity_handler: Option<String>,
}

