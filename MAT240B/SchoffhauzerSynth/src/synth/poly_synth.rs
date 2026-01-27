use crate::params::SchoffhauzerSynthPluginParams;
use crate::synth::synth::Synth;
use crate::utils::Single;
use crate::utils::db::DB;
use crate::utils::envelope::{ADSR, ADSRInstance};
use crate::utils::midi_note::MidiNote;
use crate::utils::modulated::Modulated;
use clack_plugin::events::Match;
use clack_plugin::events::event_types::{
    NoteChokeEvent, NoteEndEvent, NoteExpressionEvent, NoteOffEvent, NoteOnEvent, ParamModEvent,
    ParamValueEvent,
};
use repetitive::repetitive;
use std::collections::LinkedList;
use std::ops::Range;

pub struct HostNoteMatch {
    channel: Match<u16>,
    note: Match<u16>,
    id: Match<u32>,
}

repetitive! {
    @for event in [
        'NoteOnEvent,
        'NoteOffEvent,
        'NoteChokeEvent,
        'NoteEndEvent,
        'NoteExpressionEvent,
        'ParamValueEvent,
        'ParamModEvent,
    ] {
        impl From<&@event> for HostNoteMatch {
            fn from(value: &@event) -> Self {
                Self {
                    channel: value.channel(),
                    note: value.key(),
                    id: value.note_id(),
                }
            }
        }
    }
}

struct NoteIdentHost {
    channel: u16,
    note: MidiNote<u16>,
    id: Option<u32>,
}

enum NoteIdent {
    Host(NoteIdentHost),
    _Other(u32),
}

struct Voice {
    ident: NoteIdent,
    synth: Synth,
    volume: Modulated<Option<DB<f32>>>,
    adsr: ADSR<Modulated<Option<f32>>>,
    adsr_instance: ADSRInstance,
    hf_rolloff: Modulated<Option<f32>>,
}

impl Voice {
    fn new_host(
        _params: &SchoffhauzerSynthPluginParams,
        sample_rate: f32,
        channel: u16,
        note: u16,
        id: Option<u32>,
        _velocity: f32,
    ) -> Self {
        let note = MidiNote(note);
        Self {
            ident: NoteIdent::Host(NoteIdentHost { channel, note, id }),
            synth: Synth::new(sample_rate, note.freq()),
            volume: Modulated::new(None, None),
            adsr: ADSR::default(),
            adsr_instance: ADSRInstance::new(ADSR::default()),
            hf_rolloff: Modulated::new(None, None),
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

    fn off(&mut self, _velocity: f32) {
        self.adsr_instance.off();
    }

    fn choke(&mut self) {
        self.adsr_instance.force_end();
    }

    fn synth_add_to(&mut self, buffer: &mut [f32], params: &SchoffhauzerSynthPluginParams) -> bool {
        let volume = self.volume.unwrap_or(params.get_volume()).modulated();
        
        let adsr = self.adsr.map2(&params.get_adsr(), |a, b| a.unwrap_or(*b));
        let adsr = adsr.map(|it| it.modulated());
        self.adsr_instance.adsr = adsr;
        
        self.synth.hf_rolloff = self.hf_rolloff.unwrap_or(params.get_hf_rolloff()).modulated();

        for sample_ref in buffer {
            let mut sample = self.synth.synth();
            sample *= volume.linear();
            self.adsr_instance.advance(1.0 / self.synth.sample_rate);
            sample *= self.adsr_instance.current_level();
            *sample_ref += sample;
            if self.adsr_instance.ended() {
                return false;
            }
        }
        true
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

    pub fn handle_note_on_event(
        &mut self,
        event: &NoteOnEvent,
        params: &SchoffhauzerSynthPluginParams,
    ) {
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
                params,
                self.sample_rate,
                channel,
                key,
                event.note_id().into_specific(),
                event.velocity() as f32,
            ));
        }
    }

    pub fn handle_note_off_event(&mut self, event: &NoteOffEvent) {
        if !event.port_index().matches(0u16) {
            return;
        }

        self.for_each_matching_voice(&HostNoteMatch::from(event), |voice| {
            voice.off(event.velocity() as f32);
        });
    }

    pub fn handle_note_choke_event(&mut self, event: &NoteChokeEvent) {
        if !event.port_index().matches(0u16) {
            return;
        }

        self.for_each_matching_voice(&HostNoteMatch::from(event), |voice| {
            voice.choke();
        })
    }

    repetitive! {
        @for ty in ['value, 'modulation] {
            @let [event_name, event_type, event_method] = match ty {
                'value => ['value, 'ParamValueEvent, 'value],
                'modulation => ['mod, 'ParamModEvent, 'amount],
            };

            pub fn @['handle_param_ event_name '_event](&mut self, event: &@event_type) {
                let note_match = HostNoteMatch::from(event);
                match event.param_id() {
                    __ if __ == Some(SchoffhauzerSynthPluginParams::VOLUME.id) => {
                        self.for_each_matching_voice(&note_match, |voice| {
                            voice.volume.@ty = Some(DB(event.@event_method() as f32));
                        })
                    }
                    @for field in ['attack_duration, 'attack_power, 'decay_duration, 'decay_power, 'sustain, 'release_duration, 'release_power] {
                        __ if __ == Some(SchoffhauzerSynthPluginParams::ADSR.@field.id) => {
                            self.for_each_matching_voice(&note_match, |voice| {
                                voice.adsr.@field.@ty = Some(event.@event_method() as f32);
                            })
                        }
                    }
                    __ if __ == Some(SchoffhauzerSynthPluginParams::HF_ROLLOFF.id) => {
                        self.for_each_matching_voice(&note_match, |voice| {
                            voice.hf_rolloff.@ty = Some(event.@event_method() as f32);
                        })
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn is_busy(&self) -> bool {
        !self.voices.is_empty()
    }
}
