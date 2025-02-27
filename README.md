# batteryctl

**Batteryctl** is an open-source Rust-based command-line tool for interacting with batteries on Linux.

## Features

- Query specific properties of battery devices (e.g., capacity, status).
- Set events to run on battery levels.
- Flexible command-line interface.
- Lightweight and fast, written in Rust.

## Installation

To build and install **batteryctl**, you need Rust and Cargo installed on your system.

### Scripted install

Clone the repository and run the install script.

```Bash
git clone https://github.com/DucktectiveCZ/batteryctl.git
cd batteryctl
chmod +x ./install_linux.sh
./install_linux.sh
```

### Manual install

Clone the repository, build the project and copy the binaries into '/usr/local/bin/'.
```Bash
git clone https://github.com/DucktectiveCZ/batteryctl.git
cd batteryctl
cargo build --release
sudo cp target/release/batteryctl /usr/local/bin/
```

## Usage

The general syntax for `batteryctl` is:

```Bash
batteryctl <OPERATION> [<ARGS>]
```

### Parameters:

- `<OPERATION>`: The action to perform.
- `<ARGS>`: Additional arguments for the operation.

### Arguments:

- `--help` `-h`: Show help.
- `--device` `-d`: The device to use. [Default: `BAT0`]
- `--read-delay`: The daemon battery percentage read delay in milliseconds. [Default: `180000`]
- `--good-capacity` `-g`: The daemon good capacity percentage. [Default: `75`]
- `--okay-capacity` `-o`: The daemon okay capacity percentage. [Default: `50`]
- `--bad-capacity` `-b`: The daemon bad capacity percentage. [Default: `20`]
- `--critical-capacity` `-c`: The daemon critical capacity percentage. [Default: `5`]
- `--good-capacity-handler`: A script to run when the battery reaches the good percentage.
- `--okay-capacity-handler`: A script to run when the battery reaches the okay percentage.
- `--bad-capacity-handler`: A script to run when the battery reaches the bad percentage.
- `--critical-capacity-handler`: A script to run when the battery reaches the critical percentage.

### Examples

1. Get the capacity of `BAT0`:
   ```Bash
   batteryctl get-property --device BAT0 capacity
   ```

2. List the available devices:
   ```Bash
   batteryctl list-devices
   ```
3. Start the daemon:
    ```Bash
    batteryctl daemon --device BAT0
    ```

## License

`Batteryctl` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Feel free to contribute to the project by opening [issues](https://github.com/DucktectiveCZ/batteryctl/issues) or pull requests!

