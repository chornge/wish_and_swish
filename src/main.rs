mod audio;
mod motor;

use audio::microphone::{process_microphone_input, start_microphone};
use motor::actuator::start_actuator;

fn main() {
    let device_index = 2;
    let mut subprocess = start_microphone(device_index);

    if let Some(stdout) = subprocess.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        process_microphone_input(reader, start_actuator);
    }

    let _ = subprocess.wait();

    loop {
        std::thread::park();
    }
}
