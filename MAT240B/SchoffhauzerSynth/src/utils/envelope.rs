use crate::utils::lerp;
use core::fmt::Debug;

#[derive_aliases::derive(..Copy, Debug, derive_more::Display, Default)]
#[display(bound(T: Debug))]
#[display("{self:?}")]
pub struct ADSR<T> {
    pub attack_duration: T,
    pub attack_power: T,
    pub decay_duration: T,
    pub decay_power: T,
    pub sustain: T,
    pub release_duration: T,
    pub release_power: T,
}

impl<T> ADSR<T> {
    pub fn duration(&self, phase: ADSRPhase) -> Option<&T> {
        match phase {
            ADSRPhase::Attack => Some(&self.attack_duration),
            ADSRPhase::Decay => Some(&self.decay_duration),
            ADSRPhase::Sustain => None,
            ADSRPhase::Release => Some(&self.release_duration),
        }
    }
    
    pub fn power(&self, phase: ADSRPhase) -> Option<&T> {
        match phase {
            ADSRPhase::Attack => Some(&self.attack_power),
            ADSRPhase::Decay => Some(&self.decay_power),
            ADSRPhase::Sustain => None,
            ADSRPhase::Release => Some(&self.release_power),
        }
    }
    
    pub fn map2<B, R>(&self, other: &ADSR<B>, mut f: impl FnMut(&T, &B) -> R) -> ADSR<R> {
        ADSR {
            attack_duration: f(&self.attack_duration, &other.attack_duration),
            attack_power: f(&self.attack_power, &other.attack_power),
            decay_duration: f(&self.decay_duration, &other.decay_duration),
            decay_power: f(&self.decay_power, &other.decay_power),
            sustain: f(&self.sustain, &other.sustain),
            release_duration: f(&self.release_duration, &other.release_duration),
            release_power: f(&self.release_power, &other.release_power),
        }
    }
}

impl<T: Copy> ADSR<Option<T>> {
    pub fn unwrap_or(self, default: ADSR<T>) -> ADSR<T> {
        self.map2(&default, |a, b| a.unwrap_or(*b))
    }
}

#[derive_aliases::derive(..Copy, Debug, derive_more::Display, ..Eq)]
pub enum ADSRPhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl ADSRPhase {
    pub fn next(self) -> Option<ADSRPhase> {
        match self {
            ADSRPhase::Attack => Some(ADSRPhase::Decay),
            ADSRPhase::Decay => Some(ADSRPhase::Sustain),
            ADSRPhase::Sustain => Some(ADSRPhase::Release),
            ADSRPhase::Release => None,
        }
    }
}

pub struct ADSRInstance {
    pub adsr: ADSR<f32>,
    /// `None` means ended
    phase: Option<ADSRPhase>,
    start_level: f32,
    progress: f32,
}

impl ADSRInstance {
    pub fn new(adsr: ADSR<f32>) -> Self {
        Self {
            adsr,
            phase: Some(ADSRPhase::Attack),
            start_level: 0.0,
            progress: 0.0,
        }
    }

    pub fn current_level(&self) -> f32 {
        if self.phase == Some(ADSRPhase::Sustain) {
            return self.adsr.sustain;
        }
        if self.phase == None {
            return 0.0;
        }

        let range = match self.phase {
            Some(ADSRPhase::Attack) => 0.0..=1.0,
            Some(ADSRPhase::Decay) => self.start_level..=self.adsr.sustain,
            Some(ADSRPhase::Release) => self.start_level..=0.0,
            _ => unreachable!(),
        };
        let power = *self.adsr.power(self.phase.unwrap()).unwrap();
        lerp(range, power)
    }

    pub fn advance(&mut self, delta: f32) {
        let phase_duration = match self.phase.and_then(|phase| self.adsr.duration(phase)) {
            Some(&phase_duration) => phase_duration,
            None => return,
        };
        if phase_duration != 0.0 {
            self.progress += delta / phase_duration;
        }

        if self.progress >= 1.0 || phase_duration == 0.0 {
            self.progress -= 1.0;
            self.phase = self.phase.unwrap().next();
            self.start_level = match self.phase {
                Some(ADSRPhase::Attack) => unreachable!(),
                Some(ADSRPhase::Decay) => 1.0,
                Some(ADSRPhase::Sustain) => self.adsr.sustain,
                Some(ADSRPhase::Release) => self.adsr.sustain,
                None => 0.0,
            };
        }
    }

    pub fn off(&mut self) {
        match self.phase {
            Some(ADSRPhase::Attack) | Some(ADSRPhase::Decay) => {
                self.start_level = self.current_level();
                self.phase = Some(ADSRPhase::Release);
            }
            Some(ADSRPhase::Sustain) => {
                self.start_level = self.adsr.sustain;
                self.phase = Some(ADSRPhase::Release);
            }
            _ => {}
        };
    }

    pub fn force_end(&mut self) {
        self.phase = None;
    }

    pub fn ended(&self) -> bool {
        self.phase == None
    }
}
