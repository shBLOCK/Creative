use std::f32::consts::PI;

pub struct Synth {
    pub sample_rate: f32,
    pub freq: f32,
    pub hf_rolloff: f32,

    osc: f32,
    last_osc: f32,
    phase: f32,
    last_out: f32,
}

impl Synth {
    pub fn new(sample_rate: f32, freq: f32) -> Self {
        Self {
            sample_rate,
            freq,
            hf_rolloff: 1.0,
            osc: 0.0,
            last_osc: 0.0,
            phase: 0.0,
            last_out: 0.0,
        }
    }

    pub fn synth(&mut self) -> f32 {
        let w = self.freq / self.sample_rate;
        let n = 0.5 - w;
        let scaling = 13.0 * n.powi(4);
        let dc = 0.376 - w * 0.752;

        self.phase += 2.0 * w;
        if self.phase >= 1.0 {
            self.phase -= 2.0;
        }

        self.osc = (self.osc + f32::sin(2.0 * PI * (self.phase + self.osc * scaling * self.hf_rolloff))) * 0.5;
        let mut out = 2.5 * self.osc + -1.5 * self.last_osc;
        self.last_osc = self.osc;

        let last_out = self.last_out;
        self.last_out = out;
        out = (out + last_out) * 0.5;

        out += dc;
        out * (1.0 - 2.0 * w) // normalize
    }
}
