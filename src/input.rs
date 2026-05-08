pub fn recive_audio() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No Microphone Found");
    let config = device.default_input_config().unwrap();

    //----------

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _| {
            let _ = tx.send(data.to_vec());
        },
        move |_| {
            println!("An error has occured in stream:");
        },
        None,
    );
}
