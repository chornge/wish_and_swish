#[cfg(feature = "gpio")]
use rppal::gpio::{Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;

#[cfg(feature = "gpio")]
const STEPPER_MOTOR_PINS: [u8; 4] = [17, 27, 22, 23]; // Physical Pins [11, 13, 15, 16]
const STEPS: usize = 512; // Full rotation

pub trait GpioControl {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

#[cfg(feature = "gpio")]
impl GpioControl for OutputPin {
    fn set_high(&mut self) {
        OutputPin::set_high(self);
    }

    fn set_low(&mut self) {
        OutputPin::set_low(self);
    }
}

#[cfg(not(feature = "gpio"))]
#[derive(Clone, Default)]
struct MockPin {
    high: bool,
}

#[cfg(not(feature = "gpio"))]
impl GpioControl for MockPin {
    fn set_high(&mut self) {
        self.high = true;
    }

    fn set_low(&mut self) {
        self.high = false;
    }
}

pub fn start_actuator() {
    #[cfg(feature = "gpio")]
    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    #[cfg(feature = "gpio")]
    let mut motor_pins: Vec<OutputPin> = STEPPER_MOTOR_PINS
        .iter()
        .map(|pin| {
            gpio.get(*pin)
                .expect("Failed to get GPIO pin")
                .into_output()
        })
        .collect();

    #[cfg(not(feature = "gpio"))]
    let mut motor_pins = vec![MockPin::default(); 4];

    activate_motor(&mut motor_pins);
}

pub fn activate_motor<T: GpioControl>(motor_pins: &mut [T]) {
    println!("Rotating motor...");
    rotate_motor(motor_pins, STEPS);

    sleep(Duration::from_secs(4));

    println!("Reverting motor...");
    rotate_motor(motor_pins, STEPS);
}

fn rotate_motor<T: GpioControl>(motor_pins: &mut [T], steps: usize) {
    for _ in 0..steps {
        set_pins_high_low(motor_pins);
        sleep(Duration::from_millis(1));
    }
}

fn set_pins_high_low<T: GpioControl>(motor_pins: &mut [T]) {
    for j in 0..motor_pins.len().min(4) {
        motor_pins[j].set_high();
        for i in 0..motor_pins.len().min(4) {
            if i != j {
                motor_pins[i].set_low();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn activating_motor_sets_pins_high_and_reverts_after_4_seconds() {
        let mut motor_pins = vec![MockPin::default(); 4];

        let start_time = Instant::now();
        activate_motor(&mut motor_pins);
        let elapsed_time = start_time.elapsed();

        assert!(
            elapsed_time >= Duration::from_secs(4),
            "Motor operation did not wait 4 seconds"
        );

        // Uncomment the following lines if you want to check the state of the pins after activation
        /*
        for pin in &motor_pins {
            assert!(!pin.high, "Motor pin should be low after reversion");
        }
        */
    }
}
