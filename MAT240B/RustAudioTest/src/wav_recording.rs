use std::error::Error;
use std::{fs, io};
use std::path::Path;
use std::time::Duration;
use rodio::{ChannelCount, SampleRate, Source};
use hound::{WavSpec, WavWriter};

pub struct WavRecordingSource<S: Source, W: io::Write + io::Seek> {
    source: S,
    writer: WavWriter<W>,
}

impl<I: Source, W: io::Write + io::Seek> WavRecordingSource<I, W> {
    pub fn new(writer: W, source: I) -> Result<Self, Box<dyn Error>> {
        let spec = WavSpec {
            channels: source.channels(),
            sample_rate: source.sample_rate(),
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let writer = WavWriter::new(writer, spec)?;
        Ok(Self { source, writer })
    }

    pub fn finalize(self) -> Result<(), Box<dyn Error>> {
        Ok(self.writer.finalize()?)
    }
}

impl<I: Source> WavRecordingSource<I, io::BufWriter<fs::File>> {
    pub fn new_file(
        filename: impl AsRef<Path>,
        buffer_size: usize,
        source: I,
    ) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(filename.as_ref().parent().unwrap())?;
        Ok(Self::new(
            io::BufWriter::with_capacity(buffer_size, fs::File::create(filename)?),
            source,
        )?)
    }
}

impl<S: Source, W: io::Write + io::Seek> Iterator for WavRecordingSource<S, W> {
    type Item = rodio::Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.source.next() {
            self.writer.write_sample(sample).unwrap();
            Some(sample)
        } else {
            None
        }
    }
}

impl<S: Source, W: io::Write + io::Seek> Source for WavRecordingSource<S, W> {
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> ChannelCount {
        self.source.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}