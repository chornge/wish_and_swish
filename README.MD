# Wish n' Swish

## Overview

**Wish n' Swish** is a voice-operated trash can opener that utilizes a microcontroller and a stepper motor to automatically open a trash can when it hears the keyword "kobe." This project aims to make waste disposal hands-free and convenient.

## Features

- Voice activation via keyword detection
- Stepper motor control to operate the trash can foot pedal
- Battery-operated design with easily swappable batteries

## Components Required

- ~~Raspberry Pi (any model with GPIO support)~~ Micro:bit v2 (or any thumbv7em target)
- USB microphone or compatible GPIO microphone
- Stepper motor with driver (e.g., A4988 or DRV8825)
- Power supply (battery pack)
- Jumper wires and breadboard (for prototyping)
- Trash can with a foot pedal

## Installation

### Prerequisites

- Follow the prerequisite instructions at [https://github.com/esp-rs/esp-idf-template]

## Installation

### Clone Repository:

```bash
git clone https://github.com/chornge/wish_n_swish.git
cd wish-n-swish
```

## Usage

- Wiring: Connect stepper motor and microphone to GPIO pins accordingly.

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