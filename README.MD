# Wish n' Swish

## Overview

**Wish n' Swish** is a voice-operated trash can opener that utilizes a Raspberry Pi and a stepper motor to automatically open a trash can when it hears the keyword "kobe." This project aims to make waste disposal hands-free and convenient.

## Features

- Voice activation via keyword detection
- Stepper motor control to operate the trash can foot pedal
- Battery-operated design with easily swappable batteries

## Components Required

- Raspberry Pi (any model with GPIO support)
- USB microphone or compatible GPIO microphone
- Stepper motor with driver (e.g., A4988 or DRV8825)
- Power supply (battery pack)
- Jumper wires and breadboard (for prototyping)
- Trash can with a foot pedal

## Installation

### Prerequisites

- Rust installed on your Raspberry Pi. Follow the instructions at [rustup.rs](https://rustup.rs/) to install Rust.
- The following dependencies in your `Cargo.toml`:

```toml
[dependencies]
cpal = "0.15.3"       # For audio output
sysfs_gpio = "0.6.2"  # For GPIO access/support
vosk = "0.3.1"        # For offline speech recognition
```

## Installation

### Clone Repository:

```bash
git clone https://github.com/chornge/wish_n_swish.git
cd wish-n-swish
```

## Usage

- Wiring: Connect stepper motor and microphone to Raspberry Pi GPIO pins accordingly.

### Build Appication:

```bash
cargo build
```

### Run Application:

```bash
cargo run
```
- Activate the Trash Can: Say "kobe" to open the trash can. The motor will activate, holding the pedal down for 5 seconds.

## Troubleshooting

- Keyword Detection: If the keyword "kobe" is not detected, ensure your microphone is correctly configured and positioned.
- Motor Activation: Check your wiring and motor driver connections if the motor does not activate.
- Power Supply: Ensure that your battery is charged and capable of supplying adequate voltage and current.

## Acknowledgements

- Thanks to the open-source community for providing libraries and tools that made this project possible!