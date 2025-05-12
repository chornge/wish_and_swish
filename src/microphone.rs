use std::io::BufRead;
use std::process::{Command, Stdio};

const WAKEWORD: &str = "kobe";
const MODEL_PATH: &str = "../model.rpw";
const RUSTPOTTER_PATH: &str = "../rustpotter-cli";

pub fn start_microphone(device_index: usize) -> std::process::Child {
    Command::new(RUSTPOTTER_PATH)
        .arg("spot")
        .arg("--device-index")
        .arg(device_index.to_string())
        .arg(MODEL_PATH)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rustpotter-cli")
}

pub fn process_microphone_input<R: BufRead, F: FnMut()>(reader: R, mut wakeword_detected: F) {
    for input in reader.lines().map_while(Result::ok) {
        println!("{}", input);
        if input.contains(WAKEWORD) {
            wakeword_detected();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn input_with_wakeword_should_trigger_detection() {
        let input = format!("Input1\n{}\nInput2\n", WAKEWORD);
        let reader = Cursor::new(input);

        let mut detected = false;

        process_microphone_input(reader, || {
            detected = true;
        });

        assert!(detected, "Wakeword should've been detected in the input");
    }

    #[test]
    fn input_without_wakeword_should_not_trigger_detection() {
        const SILENCE: &str = "";
        let input = format!("Input1\n{}\nInput2\n", SILENCE);
        let reader = Cursor::new(input);

        let mut detected = false;

        process_microphone_input(reader, || {
            detected = false;
        });

        assert!(!detected, "Wakeword was incorrectly detected in the input");
    }
}
