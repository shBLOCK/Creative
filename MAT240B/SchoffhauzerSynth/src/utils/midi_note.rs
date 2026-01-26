#![allow(unused)]

use derive_more::Display;
use num_traits::Num;

#[derive_aliases::derive(..Copy, Debug, Display)]
#[display("MidiNode({_0})")]
pub struct MidiNote<T: Num + Copy>(pub T);

impl<T: Num + Copy> MidiNote<T> {
    pub const fn midi(self) -> T {
        self.0
    }
}

impl<T: Num + Copy + TryFrom<f32>> MidiNote<T> {
    pub fn try_from_freq(freq: f32) -> Result<Self, <T as TryFrom<f32>>::Error> {
        Ok(Self(freq_to_midi_note(freq).try_into()?))
    }
}

impl<T: Num + Copy + Into<f32>> MidiNote<T> {
    pub fn freq(self) -> f32 {
        midi_note_to_freq(self.0.into())
    }

    pub fn interval(self) -> f32 {
        1.0 / self.freq()
    }
}

fn midi_note_to_freq(note: f32) -> f32 {
    440.0 * f32::powf(2.0, (note - 69.0) / 12.0)
}

fn freq_to_midi_note(freq: f32) -> f32 {
    12.0 * f32::log2(freq / 440.0) + 69.0
}
