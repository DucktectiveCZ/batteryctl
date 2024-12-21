# batteryctl

**batteryctl** is a Rust-based command-line tool for interacting with battery information on Linux systems. It allows you to query properties of your battery devices quickly and efficiently.

## Features

- Query specific properties of battery devices (e.g., capacity, status).
- Lightweight and fast, written in Rust.
- Flexible command-line interface.

## Installation

To build and install **batteryctl**, you need Rust and Cargo installed on your system. Then, clone the repository and build the tool:

```bash
git clone https://github.com/DucktectiveCZ/batteryctl.git
cd batteryctl
cargo build --release
sudo cp target/release/batteryctl /usr/local/bin/
```

## Usage

The general syntax for `batteryctl` is:

```bash
batteryctl <OPERATION> <DEVICE> <ARGS>
```

### Parameters:

- `<OPERATION>`: The action to perform. Currently, the only supported operation is `get-property`.
- `<DEVICE>`: The battery device to interact with (e.g., `BAT0`).
- `<ARGS>`: Additional arguments for the operation. For `get-property`, this is the property name (e.g., `capacity`, `status`).

### Examples

1. Get the capacity of `BAT0`:
   ```bash
   batteryctl get-property BAT0 capacity
   ```

2. Get the status of `BAT1`:
   ```bash
   batteryctl get-property BAT1 status
   ```

## License

`batteryctl` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Feel free to contribute to the project by opening issues or pull requests on GitHub!
