use std::time::Duration;
use rodio::{ChannelCount, SampleRate, Source};
use nalgebra_glm::{vec1, TVec, Vec1};
use crate::const_generics_tools::{Assert, IsTrue};

pub trait AudioFunction<const CH: ChannelCount> {
    fn sample(&mut self, t: f32) -> TVec<f32, { CH as usize }>;
}

impl<F: FnMut(f32) -> f32> AudioFunction<1> for F {
    fn sample(&mut self, t: f32) -> Vec1 {
        vec1(self(t))
    }
}

impl<const CH: ChannelCount, F: FnMut(f32) -> TVec<f32, { CH as usize }>> AudioFunction<CH> for F
where
    Assert<{ CH > 1 }>: IsTrue,
{
    fn sample(&mut self, t: f32) -> TVec<f32, { CH as usize }> {
        self(t)
    }
}

pub struct AudioFunctionSource<const CH: ChannelCount, F: AudioFunction<CH>>
where
    [(); CH as usize]:,
{
    sample_rate: SampleRate,
    function: F,
    current_sample: TVec<f32, { CH as usize }>,
    index: u64,
}

impl<const CH: ChannelCount, F: AudioFunction<CH> + 'static> AudioFunctionSource<CH, F>
where
    [(); CH as usize]:,
{
    pub fn new(sample_rate: SampleRate, function: F) -> Self {
        Self {
            sample_rate,
            function,
            current_sample: TVec::zeros(),
            index: 0,
        }
    }
}

impl<const CH: ChannelCount, F: AudioFunction<CH> + 'static> Iterator for AudioFunctionSource<CH, F>
where
    [(); CH as usize]:,
{
    type Item = rodio::Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        let ch = index % CH as u64;
        if ch == 0 {
            self.current_sample = self
                .function
                .sample((index / CH as u64) as f32 / self.sample_rate as f32);
        }
        Some(self.current_sample[ch as usize])
    }
}

impl<const CH: ChannelCount, F: AudioFunction<CH> + 'static> Source for AudioFunctionSource<CH, F>
where
    [(); CH as usize]:,
{
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        CH
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}