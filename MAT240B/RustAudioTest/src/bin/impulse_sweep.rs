#![feature(generic_const_exprs)]

use RustAudioTest::simple_audio;
use std::f32::consts::{FRAC_1_SQRT_2, TAU};
use rodio::SampleRate;
use RustAudioTest::math::Note;

const SR: SampleRate = 48000;

fn main() {
    let mut theta = 0f32;
    simple_audio::play_and_record_audio_function::<SR, 2, _>("output/impulse_sweep.wav", 12.8, move |t| {
        let freq = Note::from_midi(t * 10.0).freq();
        theta += freq / SR as f32 * TAU;
        theta %= TAU;
        let bandlimit_n = SR as f32 / 2.0 / freq;
        let mut signal = 0f32;
        let mut total_amp = 0f32;
        for n in 1..10000 {
            let mut amp = 1.0;
            amp *= (bandlimit_n - n as f32).clamp(0.0, 1.0);
            if amp == 0.0 { break; }
            total_amp += amp;
            signal += f32::sin(n as f32 * theta) * amp;
        }
        signal / total_amp * FRAC_1_SQRT_2
    })
        .unwrap();
}
