use std::process::{Command, exit};

mod stt;
mod tts;

use tts::speech_079;

fn main() {
    println!("<-- SYS START -->");
    
    let status = Command::new("espeak-ng")
        .arg("eSpeak running")
        .status();

        match status {
            Ok(_) => {println!("Voice: Connected and Running")},
            Err(_) => {println!("Voice: Failed to connect - Check if eSpeak is installed"); exit(1)},
        }

        
        let running = true;
        println!("System is now listning");

        

        while running{

        }
}