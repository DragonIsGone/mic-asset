use cpal::traits::{DeviceTrait, HostTrait};
use std::process::{Command, exit};

mod stt;
mod tts;

use tts::speech_079;

fn main() {
    println!("<-- SYS START -->");

    let status = Command::new("espeak-ng").arg("eSpeak running").status();
    match status {
        Ok(_) => {
            println!("Voice: Connected and Running")
        }
        Err(_) => {
            println!("Voice: Failed to connect - Check if eSpeak is installed");
            exit(1)
        }
    }
    let output_device = get_outputs();
    match output_device.status() {
        Ok(_) => println!("Output Detected"),
        Err(_) => {
            println!("Output: Failed to connect - Check if Virtual Cable is installed");
            exit(1)
        }
    }

    let running = true;
    println!("System is now listning");
    speech_079("Test Message");

    while running {}
}

fn get_outputs() -> cpal::Device {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();

    for device in devices {
        let name = device.name().unwrap_or_default();
        if name.contains("CABLE") || name.contains("Virtual") || name.contains("Loopback") {
            return device;
        }
    }
    host.default_output_device()
        .unwrap()
}
