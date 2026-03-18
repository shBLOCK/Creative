#![feature(linked_list_cursors)]
#![feature(step_trait)]
#![feature(new_range_api)]
#![feature(lock_value_accessors)]

mod derive_alias;
mod params;
mod remote;
mod save_state;
mod synth;
mod utils;

use crate::params::SchoffhauzerSynthPluginParams;
use crate::remote::spawn_remote_thread;
use crate::synth::poly_synth::PolySynth;
use clack_extensions::audio_ports::{
    AudioPortFlags, AudioPortInfo, AudioPortInfoWriter, AudioPortType, PluginAudioPorts,
    PluginAudioPortsImpl,
};
use clack_extensions::log::{HostLog, LogSeverity};
use clack_extensions::note_ports::{
    NoteDialect, NoteDialects, NotePortInfo, NotePortInfoWriter, PluginNotePorts,
    PluginNotePortsImpl,
};
use clack_extensions::params::PluginParams;
use clack_extensions::state::PluginState;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::plugin::features::{INSTRUMENT, MONO, SYNTHESIZER};
use clack_plugin::prelude::*;
use pymeta::pymeta;
use std::ffi::CStr;

pub struct SchoffhauzerSynthPlugin;

impl Plugin for SchoffhauzerSynthPlugin {
    type AudioProcessor<'a> = SchoffhauzerSynthAudioProcessor<'a>;
    type Shared<'a> = SchoffhauzerSynthShared<'a>;
    type MainThread<'a> = SchoffhauzerSynthPluginMainThread<'a>;

    fn declare_extensions(
        builder: &mut PluginExtensions<Self>,
        _shared: Option<&Self::Shared<'_>>,
    ) {
        builder
            .register::<PluginAudioPorts>()
            .register::<PluginNotePorts>()
            .register::<PluginParams>()
            .register::<PluginState>();
    }
}

impl DefaultPluginFactory for SchoffhauzerSynthPlugin {
    fn get_descriptor() -> PluginDescriptor {
        PluginDescriptor::new("dev.shblock.schoffhauzer_synth", "Schoffhauzer Synth")
            .with_features([SYNTHESIZER, MONO, INSTRUMENT])
    }

    fn new_shared(host: HostSharedHandle<'_>) -> Result<Self::Shared<'_>, PluginError> {
        unsafe { std::env::set_var("RUST_BACKTRACE", "full"); }
        let shared = SchoffhauzerSynthShared {
            host,
            params: SchoffhauzerSynthPluginParams::default(),
            logger: host.get_extension::<HostLog>(),
        };
        Ok(shared)
    }

    fn new_main_thread<'a>(
        _host: HostMainThreadHandle<'a>,
        shared: &'a Self::Shared<'a>,
    ) -> Result<Self::MainThread<'a>, PluginError> {
        Ok(SchoffhauzerSynthPluginMainThread { shared })
    }
}

pub struct SchoffhauzerSynthAudioProcessor<'a> {
    shared: &'a SchoffhauzerSynthShared<'a>,
    synth: PolySynth,
}

impl<'a> SchoffhauzerSynthAudioProcessor<'a> {
    fn handle_event(&mut self, event: &UnknownEvent) {
        match event.as_core_event() {
            Some(CoreEventSpace::NoteOn(event)) => {
                self.synth.handle_note_on_event(event, &self.shared.params)
            }
            Some(CoreEventSpace::NoteOff(event)) => self.synth.handle_note_off_event(event),
            Some(CoreEventSpace::NoteChoke(event)) => self.synth.handle_note_choke_event(event),
            Some(CoreEventSpace::ParamValue(event)) => {
                if event.pckn().matches_all() {
                    self.shared.params.handle_param_value_event(event);
                } else {
                    self.synth.handle_param_value_event(event);
                }
            }
            Some(CoreEventSpace::ParamMod(event)) => {
                if event.pckn().matches_all() {
                    self.shared.params.handle_param_mod_event(event);
                } else {
                    self.synth.handle_param_mod_event(event);
                }
            }
            // Some(CoreEventSpace::NoteExpression(event)) => {}
            _ => {}
        }
    }
}

impl<'a> PluginAudioProcessor<'a, SchoffhauzerSynthShared<'a>, SchoffhauzerSynthPluginMainThread<'a>>
    for SchoffhauzerSynthAudioProcessor<'a>
{
    fn activate(
        _host: HostAudioProcessorHandle<'a>,
        _main_thread: &mut SchoffhauzerSynthPluginMainThread<'a>,
        shared: &'a SchoffhauzerSynthShared,
        audio_config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        shared.info(c"test");
        shared.error(c"teste");
        spawn_remote_thread(shared);
        Ok(Self {
            shared,
            synth: PolySynth::new(audio_config.sample_rate as f32),
        })
    }

    fn process(
        &mut self,
        _process: Process,
        mut audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        let mut output_port = audio
            .output_port(0)
            .ok_or(PluginError::Message("No output port found"))?;

        let mut output_channels = output_port
            .channels()?
            .into_f32()
            .ok_or(PluginError::Message("Expected f32 output"))?;

        let output_buffer = output_channels
            .channel_mut(0)
            .ok_or(PluginError::Message("Expected at least one channel"))?;
        output_buffer.fill(0.0);

        for event_batch in events.input.batch() {
            event_batch
                .events()
                .for_each(|event| self.handle_event(event));

            self.synth.synth(
                &mut output_buffer[event_batch.sample_bounds()],
                &self.shared.params,
            );
        }

        // If somehow the host didn't give us a mono output, we copy the output to all channels
        if output_channels.channel_count() > 1 {
            let (first_channel, other_channels) = output_channels.split_at_mut(1);
            let first_channel = first_channel.channel(0).unwrap();

            for other_channel in other_channels {
                other_channel.copy_from_slice(first_channel);
            }
        }

        if self.synth.is_busy() {
            Ok(ProcessStatus::Continue)
        } else {
            Ok(ProcessStatus::Sleep)
        }
    }
}

pub struct SchoffhauzerSynthShared<'a> {
    host: HostSharedHandle<'a>,
    params: SchoffhauzerSynthPluginParams,
    logger: Option<HostLog>,
}

impl SchoffhauzerSynthShared<'_> {
    fn log(&self, severity: LogSeverity, message: &CStr) {
        if let Some(logger) = &self.logger {
            logger.log(&self.host, severity, message);
        }
    }

    pymeta! {
        $for severity in ["Debug", "Info", "Warning", "Error", "Fatal"]:{
            #[allow(unused)]
            fn $severity.lower()$(&self, message: &CStr) {
                self.log(LogSeverity::$severity$, message);
            }
        }
    }
}

impl<'a> PluginShared<'a> for SchoffhauzerSynthShared<'a> {}

pub struct SchoffhauzerSynthPluginMainThread<'a> {
    shared: &'a SchoffhauzerSynthShared<'a>,
}

impl<'a> PluginMainThread<'a, SchoffhauzerSynthShared<'a>> for SchoffhauzerSynthPluginMainThread<'a> {}

impl<'a> PluginAudioPortsImpl for SchoffhauzerSynthPluginMainThread<'a> {
    fn count(&mut self, is_input: bool) -> u32 {
        if is_input { 0 } else { 1 }
    }

    fn get(&mut self, index: u32, is_input: bool, writer: &mut AudioPortInfoWriter) {
        if !is_input && index == 0 {
            writer.set(&AudioPortInfo {
                id: ClapId::new(1),
                name: b"main",
                channel_count: 1,
                flags: AudioPortFlags::IS_MAIN,
                port_type: Some(AudioPortType::MONO),
                in_place_pair: None,
            })
        }
    }
}

impl<'a> PluginNotePortsImpl for SchoffhauzerSynthPluginMainThread<'a> {
    fn count(&mut self, is_input: bool) -> u32 {
        if is_input { 1 } else { 0 }
    }

    fn get(&mut self, index: u32, is_input: bool, writer: &mut NotePortInfoWriter) {
        if is_input && index == 0 {
            writer.set(&NotePortInfo {
                id: ClapId::new(1),
                name: b"main",
                supported_dialects: NoteDialects::CLAP,
                preferred_dialect: Some(NoteDialect::Clap),
            })
        }
    }
}

clack_export_entry!(SinglePluginEntry<SchoffhauzerSynthPlugin>);
