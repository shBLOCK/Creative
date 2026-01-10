#![feature(generic_const_exprs)]

use RustAudioTest::simple_audio;
use std::f32::consts::TAU;
use rodio::SampleRate;
use RustAudioTest::math::Note;

const SR: SampleRate = 48000;

fn main() {
    let mut theta = 0f32;
    simple_audio::play_and_record_audio_function::<SR, 2, _>("output/sine_sweep.wav", 12.8, move |t| {
        theta += Note::from_midi(t * 10.0).freq() / SR as f32;
        f32::sin(theta * TAU)
    })
    .unwrap();
}
