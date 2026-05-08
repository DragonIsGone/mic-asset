use std::sync::mpsc::Receiver;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};



pub fn run_whisper_loop(reciver: Receiver<Vec<f32>>, model_path: &str) {
    let ctx = WhisperContext::new(model_path).expect("Model Failure");
    let mut state = ctx.create_state().expect("State failed to init");

    println!("Worker Ready")

    for audio_chunk in reciver{
        let mut params = FullParams::new(SamplingStrategy::Greedy);
        params.set_n_threads(4);
        params.set_language(Some("en"));

        state.full(params, &audio_chunk).expect("Infer Fault");

        let num_segments = state.full_n_segments().expect("Failed to get segments");
        for i in 0..num_segments {
            if let Ok(text) = state.full_get_segmented_text(i) {
                println!(">> {}", text);
            }
        }
        
    }
}