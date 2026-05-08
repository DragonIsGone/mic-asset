use std::path::Path;

mod input;
mod stt;

fn ensure_model_download(){
    let model_path = Path::new("ggml-base.bin");
    if !model_path.exists() {
        println!("Dwnld from Rqwest")
    }
}

fn main() {
    println!("Mic Asset");
}
