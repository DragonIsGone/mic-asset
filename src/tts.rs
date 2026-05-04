use std::process::Command;

pub fn speech_079(text: &str){
    let _ = Command::new("espeak-ng")
        .arg("-p").arg("15") //Pitch
        .arg("-s").arg("130") //Speed
        .arg(text) // <- VC Line
        .spawn()
        .expect("Error: TTS Voice engine");
}