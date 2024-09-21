use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

const STEP_PIN: u8 = 17; // GPIO pin number for stepping
const DIRECTION_PIN: u8 = 27; // GPIO pin for direction

fn step_motor(steps: u32) {
    let gpio = Gpio::new().unwrap();
    let mut step_pin = gpio.get(STEP_PIN).unwrap().into_output();
    let mut direction_pin = gpio.get(DIRECTION_PIN).unwrap().into_output();

    // Set direction (optional)
    direction_pin.set_high(); // or low for reverse

    for _ in 0..steps {
        step_pin.set_high();
        thread::sleep(Duration::from_millis(1)); // Step duration
        step_pin.set_low();
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn listen_for_keyword() {
    // Placeholder for audio listening logic
    // Use a suitable speech recognition library or API
    loop {
        // Listen for audio input and check for the keyword "kobe"
        // If detected, call the function to activate the motor
        if detect_keyword() {
            step_motor(100); // Adjust number of steps as needed
            thread::sleep(Duration::from_secs(5)); // Hold for 5 seconds
            step_motor(100); // Release
        }
    }
}

// Placeholder function for detecting the keyword
fn detect_keyword() -> bool {
    // Implement keyword detection logic here
    false
}
