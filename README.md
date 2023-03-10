# ESP32 Rust Multi Target Template

Template project for creating interactive apps with Rust Bare Metal (no_std) on ESP32.
The template contains also WASM and Desktop version which could help with faster implementation of concepts.
Each target HW is in separate directory. Shared code is stored in `engine`.

## Version of espflash

Use `espflash 2.x` which supports also targets like ESP32-C6.

Installation:

```
cargo install espflash --git https://github.com/esp-rs/espflash.git
```

## Generate new project

```
cargo generate https://github.com/georgik/esp32-rust-multi-target-template.git
cd project-name
```

## Build and flash

### Build WASM version

```
cd wasm
npm install
npm run serve
```

Open in web browser: https://localhost:8443.

Note: https is required for access to accelerometer data - https://w3c.github.io/deviceorientation/#security-and-privacy . It's possible to run the app without accelerometer on http.

### Build for ESP32-S3-BOX with ILI9486

Control: IMU
- tilt the board to move the character
- move quickly up to perform the first action
- move quickly down to perform second action

```
cd esp32-s3-box
cargo espflash flash --release --monitor
```

### Build for ESP32-C6-DevKitM-1 with ILI9341

Control: Not implemented

```
cd esp32-c6-devkit
cargo espflash flash --release --monitor
```

### Build for ESP32-C3-DeviKit-RUST with ILI9341

Control: IMU
- tilt board to move character

```
cd esp32-c3-devkit-rust
cargo espflash flash --release --monitor
```

#### Features

- Embedded Graphics
- Framebuffer
- Random maze generator
- IMU Accelerometer control

### Build for dekstop

Control: keyboard
- press arrows or W,A,S,D to move the character
- press Enter to perform action

- macOS prerequisites:
```
brew install SDL2
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

- OpenSUSE Linux prerequisites:
```
sudo zypper install SDL2-devel
```

- run:
```
cd desktop
cargo run
```

### Build for ESP32-S3-USB-OTG with ST7789

Control: buttons
- press button to move the character
- press up & down to perform the first action
- press ok & menu to perform the secomnd action

```
cd esp32-s3-usb-otg
cargo espflash flash --release --monitor
```

### Build for ESP32-S2-USB-OTG with ST7789

Control: buttons
- press button to move the character
- press up & down to perform the first action
- press ok & menu to perform the second action

```
cd esp32-s2-usb-otg
cargo espflash flash --release --monitor
```

### Build for M5STACK-FIRE with ESP32 and ILI9341

HW: https://docs.makerfactory.io/m5stack/core/fire/

Control: MPU-9250, buttons
- tilt the board to move the character
- move quickly up or press button C to perform the first action
- move quickly down or press button B to perform the second action

```
cd m5stack-fire
cargo espflash flash --release --monitor
```

#### Build M5STACK-FIRE using GitPod.io and run with Wokwi

- Open in [GitPod.io](https://gitpod.io/github.com/georgik/esp32-spooky-maze-game)

```
cd m5stack-fire
./run-wokwi.sh
```

- Wokwi project: https://wokwi.com/projects/350825213595746900

#### Build M5STACK-FIRE using Codespaces and run with Wokwi

- Navigate to [GitHub repository](https://github.com/georgik/esp32-spooky-maze-game)
- Click Open, select Codespaces tab, click Create Codespace

```
cd m5stack-fire
./run-wokwi.sh
```

#### Build M5STACK-FIRE and run Wokwi in local VS Code

Preview: install VS Code Wokwi plugin (private beta available on request)

```
cd m5stack-fire
cargo build --release --no-default-features --features "wokwi"
```

Press F1, select Wokwi: Start simulation

### Build for M5STACK-CORE2 with ESP32 and ILI9342C

HW: https://shop.m5stack.com/products/m5stack-core2-esp32-iot-development-kit

Control: MPU6886

```
cd m5stack-core2
cargo espflash flash --release --monitor
```

### Build for ESP32-S2-Kaluga v1.3

HW: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-wrover-kit.html

Control: buttons (partialy implemented based on of https://github.com/espressif/esp-bsp/blob/master/esp32_s2_kaluga_kit/esp32_s2_kaluga_kit.c#L59)
- more details https://github.com/espressif/esp-bsp/blob/master/esp32_s2_kaluga_kit/include/bsp/esp32_s2_kaluga_kit.h#L299
- K3-K6 to move the character
- (not supported) press K5 button to perform the first action
- (not supported) press K6 button to perform the second action

```
cd esp32-s2-kaluga
cargo espflash flash --release --monitor
```

Note for older version 1.2 - GPIO6 is used to control backlight.

### Build for ESP Wrover Kit

HW: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-wrover-kit.html

Control: limited, only one button available
- it's not possible to move the character
- press button Boot to to perform the first action

```
cd esp-wrover-kit
cargo espflash flash --release --monitor
```

### Build for ESP32-S2 with ILI9341

See tag v0.1.0.

```
cargo espflash flash --release --target xtensa-esp32s2-none-elf --features esp32s2_ili9341 --monitor
```

## Development

Following steps are useful for IDE integration, so that IDE can recognize which is your current target and fature set.

Check `target` configurad in the file `.cargo/config.toml`.
It should be one of following values:
```
target = "xtensa-esp32-none-elf"
target = "xtensa-esp32s2-none-elf"
target = "xtensa-esp32s3-none-elf"
target = "riscv32imac-unknown-none-elf"
```

If no value is selected, make sure to specify target on command line.

Check default `features` in `Cargo.toml`. Make sure that default set contains your board and display combinations.

If no value is selected, make sure to specify features on command line.

