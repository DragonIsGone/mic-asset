use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Stream};

pub fn recive_audio() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No Microphone Found");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("Config Died -> RX audio/input");

    let supported_configs = supported_configs_range
        .next()
        .expect("Config died -> range.next, no supported configs?")
        .with_max_sample_rate();

    //----------

    let stream = device.build_output_stream(
        &stream_config
            .0,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {},
        move |err| {
            println!("An error has occured in stream");
        },
        None,
    );
}
