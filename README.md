# Teensy 4.1 LED Blink with RTIC

Rust firmware for Teensy 4.1 that blinks the built-in LED using RTIC framework and periodic interrupt timer.

## Prerequisites

### 1. Install Rust Toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add ARM target
rustup target add thumbv7em-none-eabihf
rustup component add rust-src llvm-tools-preview

# Install cargo-binutils
cargo install cargo-binutils
```

### 2. Install Teensy Loader CLI

```bash
# Install dependencies
sudo apt install libusb-dev git build-essential

# Build and install teensy_loader_cli
git clone https://github.com/PaulStoffregen/teensy_loader_cli.git
cd teensy_loader_cli
make
sudo cp teensy_loader_cli /usr/local/bin/
cd ..
```

### 3. Set up USB Permissions

```bash
# Create udev rules
sudo tee /etc/udev/rules.d/00-teensy.rules > /dev/null <<'EOF'
ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="04[789B]?", ENV{ID_MM_DEVICE_IGNORE}="1", ENV{ID_MM_PORT_IGNORE}="1"
ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="04[789A]?", ENV{MTP_NO_PROBE}="1"
SUBSYSTEMS=="usb", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="04[789ABCD]?", MODE:="0666"
KERNEL=="ttyACM*", ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="04[789B]?", MODE:="0666", ENV{ID_MM_DEVICE_IGNORE}="1", ENV{ID_MM_PORT_IGNORE}="1"
EOF

# Reload rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Add user to dialout group (log out and back in after this)
sudo usermod -a -G dialout $USER
```

## Building

```bash
# Release build (recommended)
cargo build --release

# Debug build
cargo build
```

## Flashing

```bash
# 1. Convert to HEX format
cargo objcopy --release -- -O ihex target/teensy41.hex

# 2. Press the white button on your Teensy 4.1 board

# 3. Flash
teensy_loader_cli --mcu=TEENSY41 -w target/teensy41.hex
```

### One-Line Flash

```bash
cargo build --release && cargo objcopy --release -- -O ihex target/teensy41.hex && teensy_loader_cli --mcu=TEENSY41 -w target/teensy41.hex
```

(Press the button when prompted)

## Troubleshooting

### `cargo objcopy` not found
```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### Permission denied errors
- Make sure udev rules are installed (see step 3 above)
- Log out and back in after adding your user to dialout group
- Try: `sudo usermod -a -G dialout $USER`

### Teensy not detected
- Check USB cable (must be data-capable, not charge-only)
- Press the white button on Teensy to enter bootloader mode
- Verify with: `lsusb | grep -i teensy`

### Build errors
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

## Expected Result

After flashing, the LED on pin 13 will blink every 250ms.

## Project Structure

```
teensy41/
├── .cargo/config.toml    # Build configuration
├── Cargo.toml            # Dependencies
├── build.rs              # Build script
└── src/main.rs           # Application code
```

## License

GPL-3.0-only