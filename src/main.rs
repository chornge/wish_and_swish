use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::park;

const WAKEWORD: &str = "kobe";
const MODEL_PATH: &str = "./model.rpw";
const RUSTPOTTER_PATH: &str = "./rustpotter-cli";

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
    println!("Interacting with SERVO...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
}
