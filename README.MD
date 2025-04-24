![CI/CD](https://github.com/chornge/wish_and_swish/actions/workflows/build.yml/badge.svg?branch=main)

# Wish n' Swish

## Overview

**Wish n' Swish** is a voice-operated trash can opener that utilizes a microcontroller and a stepper motor to automatically open a trash can when it hears the keyword "kobe." This project aims to make waste disposal hands-free and convenient.

## Features

- Voice activation via keyword detection
- Stepper motor control to operate the trash can foot pedal
- Battery-operated design with easily swappable batteries

## Components Required

- ~~Micro:bit v2 (or any thumbv7em target)~~ Raspberry Pi (any model with GPIO support)
- USB microphone or compatible GPIO microphone
- Stepper motor with driver (e.g., A4988 or DRV8825)
- Power supply (battery pack)
- Jumper wires and breadboard (for prototyping)
- Trash can with a foot pedal

## Installation

~~- Follow the prerequisite [instructions](https://github.com/esp-rs/esp-idf-template)~~

### Clone Repository:

```bash
git clone https://github.com/chornge/wish_and_swish.git
cd wish_and_swish
```

### Usage (Dev machine)

- Install [RaspberryPi Imager](https://www.raspberrypi.com/software/)
- Choose Device, O.S and Storage
- Edit settings to set username, password, hostname, enable wifi and SSH
- SSH into RaspberryPi: `ssh raspberry@raspberrypi.local`
- Enter password when prompted

### Usage (RaspberryPi)

- Install the following in `/home/pi`:

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  `sudo apt install -y libclang-dev libasound2-dev`

  `rustup target add aarch64-unknown-linux-gnu`

  `rustup component add rustfmt clippy llvm-tools`

  `cargo install cargo-binutils`

### Build & Run Appication:

```bash
cargo build --release
cargo run --release
```

Say "kobe" to open the trash can - the motor will activate, holding the pedal down for 5 seconds.

## Troubleshooting

- Keyword Detection: If the keyword "kobe" is not detected, ensure your microphone is correctly configured and positioned.
- Motor Activation: Check your wiring and motor driver connections if the motor does not activate.
- Power Supply: Ensure that your battery is charged and capable of supplying adequate voltage and current.

## Acknowledgements

- Thanks to the open-source community for providing libraries and tools that made this project possible!
