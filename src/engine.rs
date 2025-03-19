use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};
use vosk::{Model, Recognizer};

const STEP_PIN: u64 = 17; // GPIO pin for stepping
const DIRECTION_PIN: u64 = 27; // GPIO pin for direction
const KEY_PHRASE: &str = "kobe";

pub fn listen_for_keyphrase() {
    let model = Model::new(".model").expect("Could not create model");
    let mut recognizer = Recognizer::new(&model, 16000.0).expect("Could not create recognizer");

    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("No input device available");

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let samples: Vec<i16> = data.iter().map(|&s| (s * 32767.0) as i16).collect();
                if recognizer.accept_waveform(&samples).is_ok() {
                    let result = recognizer.partial_result();
                    if result.partial.contains(KEY_PHRASE) {
                        println!("Keyphrase detected");
                        activate_foot_pedal();
                    }
                }
            },
            move |err| {
                eprintln!("Error occurred on stream: {}", err);
            },
            None,
        )
        .expect("Failed to build input stream");

    stream.play().expect("Failed to start input stream");

    // Keep main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn activate_foot_pedal() {
    let step_pin = Pin::new(STEP_PIN);
    let direction_pin = Pin::new(DIRECTION_PIN);

    step_pin
        .with_exported(|| {
            direction_pin.with_exported(|| {
                direction_pin.set_direction(Direction::Out)?;
                step_pin.set_direction(Direction::Out)?;

                // Set direction (optional)
                direction_pin.set_value(1)?; // or 0 for reverse

                step_pin.set_value(1)?;
                println!("Foot pedal pressed");
                thread::sleep(Duration::from_secs(5));
                step_pin.set_value(0)?;
                println!("Foot pedal released");

                Ok(())
            })
        })
        .unwrap();
}
