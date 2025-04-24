use cpal::traits::{DeviceTrait, HostTrait};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

const WAKEWORD: &str = "kobe";
const MODEL_PATH: &str = "./model.rpw";
const RUSTPOTTER_PATH: &str = "./rustpotter-cli";

fn main() {
    let device_index = 2;

    // Run the rustpotter-cli binary as a subprocess
    let mut child = Command::new(RUSTPOTTER_PATH)
        .arg("spot")
        .arg("--device-index")
        .arg(device_index.to_string())
        .arg(MODEL_PATH)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rustpotter-cli");

    // Capture output of rustpotter-cli
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        // Process each line of output
        for output in reader.lines().map_while(Result::ok) {
            println!("Rustpotter Output: {}", output);

            // Check if the output indicates a wake word detection
            if output.contains(WAKEWORD) {
                on_wakeword_detected();
            }
        }
    }

    // Wait for subprocess to finish
    let _ = child.wait();

    // Keep program running
    loop {
        thread::park();
    }
}

fn on_wakeword_detected() {
    println!("Wake word detected! Moving to SERVO state...");
}

fn _list_audio_devices() {
    let host = cpal::default_host();
    let devices = host.input_devices().expect("Failed to get input devices");
    for (index, device) in devices.enumerate() {
        println!(
            "Device {}: {}",
            index,
            device.name().unwrap_or("Unknown".to_string())
        );
    }
}
