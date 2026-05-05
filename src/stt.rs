use vosk::{Model, Recognizer};

pub fn bit_test() {
    let model = Model::new("model").expect("Model Not foud in ~/models");

    let recognizer = Recognizer::new(&model, 16000.0).expect("Failed to create recogniser");

    println!("STT: Model and Recogniser");
}

pub fn rec_input() -> String {}
