#![feature(generic_const_exprs)]

use RustAudioTest::simple_audio;
use std::f32::consts::TAU;
use rodio::SampleRate;

const SR: SampleRate = 48000;

fn main() {
    simple_audio::play_and_record_audio_function::<SR, 2, _>("output/sine.wav", 1.0, |t| {
        f32::sin(t * TAU * 440.0)
    })
    .unwrap();
}
