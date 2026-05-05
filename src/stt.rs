use std::sync::{Arc, Mutex};

use vosk::{Model, Recognizer};
use cpal::{Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

pub fn bit_test() {
    let model = Model::new("model").expect("Model Not foud in ~/models");

    let recogniser = Recognizer::new(&model, 16000.0).expect("Failed to create recogniser");

    println!("STT: Model and Recogniser -> OK");

    let recogniser = Arc::new(Mutex::new(recogniser));
    let rec_clone = Arc::clone(&recogniser);

    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let config = device.default_input_config()?;


    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let i16_sample: Vec<i16> = data.iter().map(|&s| (s * i16::MAX as f32) as i16).collect();

            let mut rec = rec_clone.lock().unwrap();
            if rec.accept_waveform(data)
        }
    )

}

pub fn rec_input(model: &Model, recognizer: &Recognizer) -> String {
    recognizer.set_words(true);


}
