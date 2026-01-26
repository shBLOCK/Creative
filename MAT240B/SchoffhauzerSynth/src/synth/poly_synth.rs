use crate::params::SchoffhauzerSynthPluginParams;
use crate::synth::synth::Synth;
use crate::utils::db::DB;
use crate::utils::midi_note::MidiNote;
use crate::utils::modulated::Modulated;
use crate::utils::Single;
use clack_plugin::events::event_types::{
    NoteChokeEvent, NoteEndEvent, NoteExpressionEvent, NoteOffEvent, NoteOnEvent, ParamModEvent,
    ParamValueEvent,
};
use clack_plugin::events::Match;
use std::collections::LinkedList;
use std::ops::Range;

pub struct HostNoteMatch {
    channel: Match<u16>,
    note: Match<u16>,
    id: Match<u32>,
}

macro_rules! impl_host_note_match_from_event {
    ($event: path) => {
        impl From<&$event> for HostNoteMatch {
            fn from(value: &$event) -> Self {
                Self {
                    channel: value.channel(),
                    note: value.key(),
                    id: value.note_id(),
                }
            }
        }
    };
}

impl_host_note_match_from_event!(NoteOnEvent);
impl_host_note_match_from_event!(NoteOffEvent);
impl_host_note_match_from_event!(NoteChokeEvent);
impl_host_note_match_from_event!(NoteEndEvent);
impl_host_note_match_from_event!(NoteExpressionEvent);
impl_host_note_match_from_event!(ParamValueEvent);
impl_host_note_match_from_event!(ParamModEvent);

struct NoteIdentHost {
    channel: u16,
    note: MidiNote<u16>,
    id: Option<u32>,
}

enum NoteIdent {
    Host(NoteIdentHost),
    Other(u32),
}

struct Voice {
    ident: NoteIdent,
    synth: Synth,
    volume: Modulated<Option<DB<f32>>>,
    _on: bool,
}

impl Voice {
    fn new_host(sample_rate: f32, channel: u16, note: u16, id: Option<u32>, velocity: f32) -> Self {
        let note = MidiNote(note);
        Self {
            ident: NoteIdent::Host(NoteIdentHost { channel, note, id }),
            synth: Synth::new(sample_rate, note.freq()),
            volume: Modulated::new(None, None),
            _on: true,
        }
    }

    fn match_host(&self, mat: &HostNoteMatch) -> bool {
        if let NoteIdent::Host(ident) = &self.ident {
            let id_matches = if let Some(id) = ident.id {
                mat.id.matches(id)
            } else {
                mat.id.is_all()
            };
            id_matches && mat.channel.matches(ident.channel) && mat.note.matches(ident.note.midi())
        } else {
            false
        }
    }

    fn off(&mut self, velocity: f32) {
        self._on = false;
    }

    fn synth_add_to(&mut self, buffer: &mut [f32], params: &SchoffhauzerSynthPluginParams) -> bool {
        let volume = self.volume.unwrap_or(params.get_volume()).modulated();
        for sample_ref in buffer {
            let mut sample = self.synth.synth();
            sample *= volume.linear();
            *sample_ref += sample;
        }
        self._on
    }
}

pub struct PolySynth {
    sample_rate: f32,

    voices: LinkedList<Voice>,
}

impl PolySynth {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,

            voices: LinkedList::new(),
        }
    }

    pub fn synth(&mut self, buffer: &mut [f32], params: &SchoffhauzerSynthPluginParams) {
        let mut cursor = self.voices.cursor_front_mut();
        while let Some(voice) = cursor.current() {
            if voice.synth_add_to(buffer, params) {
                cursor.move_next();
            } else {
                cursor.remove_current();
            }
        }
    }

    fn for_each_matching_voice(&mut self, mat: &HostNoteMatch, f: impl FnMut(&mut Voice)) {
        self.voices
            .iter_mut()
            .filter(|voice| voice.match_host(mat))
            .for_each(f)
    }

    pub fn handle_note_on_event(&mut self, event: &NoteOnEvent) {
        if !event.port_index().matches(0u16) {
            return;
        }

        let channel = event.channel().into_specific().unwrap_or(0);
        let keys = event
            .key()
            .into_specific()
            .map(Range::single)
            .unwrap_or(0..128);

        for key in keys {
            self.voices.push_back(Voice::new_host(
                self.sample_rate,
                channel,
                key,
                event.note_id().into_specific(),
                event.velocity() as f32,
            ))
        }
    }

    pub fn handle_note_off_event(&mut self, event: &NoteOffEvent) {
        if !event.port_index().matches(0u16) {
            return;
        }

        self.for_each_matching_voice(&HostNoteMatch::from(event), |voice| {
            voice.off(event.velocity() as f32)
        });
    }

    pub fn handle_param_value_event(&mut self, event: &ParamValueEvent) {
        match event.param_id() {
            Some(SchoffhauzerSynthPluginParams::ID_VOLUME) => self
                .for_each_matching_voice(&HostNoteMatch::from(event), |voice| {
                    voice.volume.value = Some(DB(event.value() as f32))
                }),
            _ => {}
        }
    }

    pub fn handle_param_mod_event(&mut self, event: &ParamModEvent) {
        match event.param_id() {
            Some(SchoffhauzerSynthPluginParams::ID_VOLUME) => self
                .for_each_matching_voice(&HostNoteMatch::from(event), |voice| {
                    voice.volume.modulation = Some(DB(event.amount() as f32))
                }),
            _ => {}
        }
    }

    pub fn is_busy(&self) -> bool {
        !self.voices.is_empty()
    }
}