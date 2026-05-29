# simple-joy-control

A minimal Rust ESP32 project for experimenting with joystick-style input.

## Hardware target

- Chip: ESP32 (Xtensa)
- Joystick button input: GPIO 0 with the internal pull-up enabled

The application logs whether the joystick button is pressed every 500 ms. The
button is treated as active-low, which is common for simple joystick modules.

## Prerequisites

Install the Espressif Rust toolchain and flashing tools:

```sh
cargo install espup espflash ldproxy
espup install
```

After installing the toolchain, source the environment file printed by `espup`
before building or flashing.

## Build

```sh
cargo build
```

## Flash and monitor

```sh
cargo run
```
