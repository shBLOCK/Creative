#[derive(Clone, Copy)]
pub struct Note(f32);

impl Note {
    pub const fn from_midi(note: f32) -> Self {
        Self(note)
    }

    pub fn from_freq(freq: f32) -> Self {
        Self(12.0 * f32::log2(freq / 440.0) + 69.0)
    }

    pub const fn midi(self) -> f32 {
        self.0
    }

    pub fn freq(self) -> f32 {
        440.0 * f32::powf(2.0, (self.midi() - 69.0) / 12.0)
    }

    pub fn interval(self) -> f32 {
        1.0 / self.freq()
    }
}