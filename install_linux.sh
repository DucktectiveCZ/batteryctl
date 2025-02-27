#!/bin/bash

echo "Installing Batteryctl on Linux..."

if ! type cargo &> /dev/null; then
    echo "Cargo has to be installed! Install it using \`apt install cargo\` on debian, \`pacman -S rust\` on arch or \`dnf install cargo\` on fedora."
    exit 1
fi

if ! cargo build --release; then
    echo "Build failed, exiting..."
    exit 2
fi

if ! cp './target/release/batteryctl' '/usr/local/bin/'; then
    echo "Copying binaries failed, exiting..."
    exit 3
fi

echo "Batteryctl was successfully installed on your system! For the usage, read the docs or run \`batteryctl --help\`."
echo "Thanks for using batteryctl! :D"
