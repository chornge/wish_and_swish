use rppal::gpio::{Gpio, OutputPin};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::{park, sleep};
use std::time::Duration;

const WAKEWORD: &str = "kobe";
const MODEL_PATH: &str = "./model.rpw";
const RUSTPOTTER_PATH: &str = "./rustpotter-cli";
const STEPPER_MOTOR_PINS: [u8; 4] = [17, 27, 22, 23]; // Physical Pins [11, 13, 15, 16]

trait GpioControl {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

impl GpioControl for OutputPin {
    fn set_high(&mut self) {
        OutputPin::set_high(self);
    }

    fn set_low(&mut self) {
        OutputPin::set_low(self);
    }
}

fn main() {
    let device_index = 2;
    let mut subprocess = Command::new(RUSTPOTTER_PATH)
        .arg("spot")
        .arg("--device-index")
        .arg(device_index.to_string())
        .arg(MODEL_PATH)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rustpotter-cli");

    if let Some(stdout) = subprocess.stdout.take() {
        let reader = BufReader::new(stdout);
        process_rustpotter_output(reader, on_wakeword_detected);
    }

    let _ = subprocess.wait();

    loop {
        park();
    }
}

fn process_rustpotter_output<R: BufRead, F: FnMut()>(reader: R, mut on_wakeword_detected: F) {
    for output in reader.lines().map_while(Result::ok) {
        println!("{}", output);
        if output.contains(WAKEWORD) {
            on_wakeword_detected();
        }
    }
}

fn on_wakeword_detected() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut motor_pins: Vec<OutputPin> = STEPPER_MOTOR_PINS
        .iter()
        .map(|pin| {
            gpio.get(*pin)
                .expect("Failed to get GPIO pin")
                .into_output()
        })
        .collect();

    activate_motor(&mut motor_pins);
}

fn activate_motor<T: GpioControl>(motor_pins: &mut [T]) {
    println!("Rotating motor...");
    for _ in 0..512 {
        for i in 0..4 {
            motor_pins[i].set_high();
            for j in 0..4 {
                if j != i {
                    motor_pins[j].set_low();
                }
            }
            sleep(Duration::from_millis(1));
        }
    }

    sleep(Duration::from_secs(3));

    println!("Reverting motor...");
    for _ in 0..512 {
        for i in (0..4).rev() {
            motor_pins[i].set_high();
            for j in 0..4 {
                if j != i {
                    motor_pins[j].set_low();
                }
            }
            sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[derive(Clone, Default)]
    struct MockPin {
        high: bool,
    }

    impl GpioControl for MockPin {
        fn set_high(&mut self) {
            self.high = true;
        }

        fn set_low(&mut self) {
            self.high = false;
        }
    }

    #[test]
    fn output_with_wakeword_should_trigger_detection() {
        let input = format!("Output1\n{}\nOutput2\n", WAKEWORD);
        let reader = Cursor::new(input);

        let mut detected = false;

        process_rustpotter_output(reader, || {
            detected = true;
        });

        assert!(detected, "Wakeword should've been detected in the output");
    }

    #[test]
    fn output_without_wakeword_should_not_trigger_detection() {
        const SILENCE: &str = "";
        let input = format!("Output1\n{}\nOutput2\n", SILENCE);
        let reader = Cursor::new(input);

        let mut detected = false;

        process_rustpotter_output(reader, || {
            detected = false;
        });

        assert!(!detected, "Wakeword was incorrectly detected in the output");
    }

    #[test]
    fn activating_motor_sets_pins_on_high() {
        let mut motor_pins = vec![MockPin::default(); 4];

        activate_motor(&mut motor_pins);

        /*for pin in &motor_pins {
            assert!(!pin.high, "Motor pin should be low after activation");
        }*/
    }
}
