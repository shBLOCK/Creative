use crate::audio_function::{AudioFunction, AudioFunctionSource};
use crate::wav_recording::WavRecordingSource;
use rodio::cpal::SampleFormat;
use rodio::{ChannelCount, SampleRate, Source};
use std::error::Error;
use std::path::Path;
use std::time::Duration;

pub fn play_and_record<const SR: SampleRate, const CH: ChannelCount>(
    filename: impl AsRef<Path>,
    source: impl Source + Send + 'static,
) -> Result<(), Box<dyn Error>>
where
    [(); CH as usize]:,
{
    let buffer_size = if let Some(duration) = source.total_duration() {
        ((duration.as_secs_f64() * (4 * SR as u32 * CH as u32) as f64 / 1024.0).ceil() as usize + 1)
            * 1024
    } else {
        8 * 1024 * 1024
    };
    let mut source = WavRecordingSource::new_file(filename, buffer_size, source)?;

    {
        let mut output_stream = rodio::OutputStreamBuilder::from_default_device()?
            .with_sample_rate(SR)
            .with_channels(CH)
            .with_sample_format(SampleFormat::F32)
            .open_stream()?;

        let sink = rodio::Sink::connect_new(output_stream.mixer());
        sink.append(unsafe { &mut *(&raw mut source) });
        sink.sleep_until_end();

        output_stream.log_on_drop(false);
    }

    source.finalize()?;

    Ok(())
}

pub fn play_and_record_audio_function<
    const SR: SampleRate,
    const CH: ChannelCount,
    const FCH: ChannelCount,
>(
    filename: impl AsRef<Path>,
    duration: f32,
    function: impl AudioFunction<FCH> + 'static + Send,
) -> Result<(), Box<dyn Error>>
where
    [(); CH as usize]:,
    [(); FCH as usize]:,
{
    play_and_record::<SR, CH>(filename, AudioFunctionSource::new(SR, function).take_duration(Duration::from_secs_f32(duration)))
}
