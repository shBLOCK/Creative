use crate::utils::db::DB;
use crate::utils::envelope::ADSR;
use crate::utils::modulated::Modulated;
use crate::{SchoffhauzerSynthAudioProcessor, SchoffhauzerSynthPluginMainThread};
use clack_extensions::params::{
    ParamDisplayWriter, ParamInfo, ParamInfoFlags, ParamInfoWriter, PluginAudioProcessorParams,
    PluginMainThreadParams,
};
use clack_plugin::events::event_types::{ParamModEvent, ParamValueEvent};
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::events::UnknownEvent;
use clack_plugin::prelude::{ClapId, InputEvents, OutputEvents};
use clack_plugin::utils::Cookie;
use repetitive::repetitive;
use static_assertions::assert_impl_all;
use std::f64;
use std::ffi::CStr;
use std::fmt::Write as _;
use std::str::FromStr;
use std::sync::RwLock;

pub trait ParamInfoFlagsExt {
    const IS_AUTOMATABLE_ALL: ParamInfoFlags = ParamInfoFlags::from_bits_truncate(
        ParamInfoFlags::IS_AUTOMATABLE.bits()
            | ParamInfoFlags::IS_AUTOMATABLE_PER_CHANNEL.bits()
            | ParamInfoFlags::IS_AUTOMATABLE_PER_KEY.bits()
            | ParamInfoFlags::IS_AUTOMATABLE_PER_NOTE_ID.bits()
            | ParamInfoFlags::IS_AUTOMATABLE_PER_PORT.bits(),
    );
    const IS_MODULATABLE_ALL: ParamInfoFlags = ParamInfoFlags::from_bits_truncate(
        ParamInfoFlags::IS_MODULATABLE.bits()
            | ParamInfoFlags::IS_MODULATABLE_PER_CHANNEL.bits()
            | ParamInfoFlags::IS_MODULATABLE_PER_KEY.bits()
            | ParamInfoFlags::IS_MODULATABLE_PER_NOTE_ID.bits()
            | ParamInfoFlags::IS_MODULATABLE_PER_PORT.bits(),
    );
    const IS_AUTOMATABLE_AND_MODULATABLE_ALL: ParamInfoFlags = ParamInfoFlags::from_bits_truncate(
        Self::IS_AUTOMATABLE_ALL.bits() | Self::IS_MODULATABLE_ALL.bits(),
    );
}

impl ParamInfoFlagsExt for ParamInfoFlags {}

// trait PluginParam: Send + Sync {
//     fn id(&self) -> ClapId;
//     fn get_raw(&self) -> f64;
//     fn set_raw(&self, value: f64);
// }
//
// struct PluginParamModule {
//
// }
//
// impl PluginParamModule {
//
// }
//
// assert_impl_all!(PluginParamModule: Send, Sync);
//
// trait PluginParams: Send + Sync {
//     fn get_param(&self, id: ClapId) -> Option<&dyn PluginParam> {
//
//     }
// }

pub struct SchoffhauzerSynthPluginParams {
    pub volume: RwLock<Modulated<DB<f32>>>,
    pub adsr: RwLock<ADSR<Modulated<f32>>>,
}

type Params = SchoffhauzerSynthPluginParams;

assert_impl_all!(SchoffhauzerSynthPluginParams: Send, Sync);

impl Default for SchoffhauzerSynthPluginParams {
    fn default() -> Self {
        Self {
            volume: RwLock::new(Modulated::new(
                DB(Params::VOLUME.default_value as f32),
                DB(0.0),
            )),
            adsr: repetitive! {
                RwLock::new(ADSR {
                    @for field in ['attack_duration, 'attack_power, 'decay_duration, 'decay_power, 'sustain, 'release_duration, 'release_power] {
                        @field: Modulated::new(Params::ADSR.@field.default_value as f32, 0.0),
                    }
                })
            },
        }
    }
}

macro_rules! join_str {
    ([$($first:literal $(, $rest:literal)*)?] ~ $join:literal) => {
        concat!($($first $(, $join, $rest)*)?)
    };
}

macro_rules! param_info {
    (id $id:literal, $($($module:literal)/+)?@$name:literal, $default:literal in $min:literal..=$max:literal $(, $($flags:ident)|+)?) => {
        ParamInfo {
            id: ClapId::new($id),
            flags: ParamInfoFlags::from_bits_truncate(0 $($(| ParamInfoFlags::$flags.bits())+)?),
            cookie: Cookie::empty(),
            name: concat!($name).as_bytes(),
            module: join_str!([$($($module),+)?] ~ "/").as_bytes(),
            min_value: $min,
            max_value: $max,
            default_value: $default,
        }
    };
}

impl SchoffhauzerSynthPluginParams {
    pub const VOLUME: &ParamInfo<'static> =
        &param_info!(id 0, @"Volume", 0.0 in -60.0..=12.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL);
    pub const ADSR: ADSR<&ParamInfo<'static>> = ADSR {
        attack_duration: &param_info!(id 1, "ADSR"@"Attack Duration", 0.1 in 0.0..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        attack_power: &param_info!(id 2, "ADSR"@"Attack Power", 0.7 in 0.2..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        decay_duration: &param_info!(id 3, "ADSR"@"Decay Duration", 0.3 in 0.0..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        decay_power: &param_info!(id 4, "ADSR"@"Decay Power", 0.7 in 0.2..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        sustain: &param_info!(id 5, "ADSR"@"Sustain", 0.5 in 0.0..=2.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        release_duration: &param_info!(id 6, "ADSR"@"Release Duration", 0.3 in 0.0..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
        release_power: &param_info!(id 7, "ADSR"@"Release Power", 0.7 in 0.2..=5.0, IS_AUTOMATABLE_AND_MODULATABLE_ALL),
    };

    pub fn get_volume(&self) -> Modulated<DB<f32>> {
        *self.volume.read().unwrap()
    }

    pub fn get_adsr(&self) -> ADSR<Modulated<f32>> {
        *self.adsr.read().unwrap()
    }

    repetitive! {
        @for ty in ['value, 'modulation] {
            @let [event_name, event_type, event_method] = match ty {
                'value => ['value, 'ParamValueEvent, 'value],
                'modulation => ['mod, 'ParamModEvent, 'amount],
            };

            pub fn @['handle_param_ event_name '_event](&self, event: &@event_type) {
                match event.param_id() {
                    __ if __ == Some(Self::VOLUME.id) => {
                        self.volume.write().unwrap().@ty = DB(event.@event_method() as f32);
                    }
                    @for field in ['attack_duration, 'attack_power, 'decay_duration, 'decay_power, 'sustain, 'release_duration, 'release_power] {
                        __ if __ == Some(Self::ADSR.@field.id) => {
                            self.adsr.write().unwrap().@field.@ty = event.@event_method() as f32;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_event(&self, event: &UnknownEvent) -> bool {
        match event.as_core_event() {
            Some(CoreEventSpace::ParamValue(event)) => self.handle_param_value_event(event),
            Some(CoreEventSpace::ParamMod(event)) => self.handle_param_mod_event(event),
            _ => return false,
        }
        true
    }
}

impl<'a> PluginMainThreadParams for SchoffhauzerSynthPluginMainThread<'a> {
    fn count(&mut self) -> u32 {
        1 + 7
    }

    fn get_info(&mut self, param_index: u32, info: &mut ParamInfoWriter) {
        let mut i = 0u32..;
        if param_index == i.next().unwrap() {
            info.set(Params::VOLUME);
        }
        repetitive! {
            @for field in ['attack_duration, 'attack_power, 'decay_duration, 'decay_power, 'sustain, 'release_duration, 'release_power] {
                if param_index == i.next().unwrap() {
                    info.set(Params::ADSR.@field);
                }
            }
        }
    }

    fn get_value(&mut self, param_id: ClapId) -> Option<f64> {
        repetitive! {
            match param_id {
                __ if __ == Some(Params::VOLUME.id) => {
                    Some(self.shared.params.get_volume().value.db() as f64)
                }
                @for field in ['attack_duration, 'attack_power, 'decay_duration, 'decay_power, 'sustain, 'release_duration, 'release_power] {
                    __ if __ == Some(Params::ADSR.@field.id) => {
                        Some(self.shared.params.get_adsr().@field.value as f64)
                    }
                }
                _ => None,
            }
        }
    }

    fn value_to_text(
        &mut self,
        param_id: ClapId,
        value: f64,
        writer: &mut ParamDisplayWriter,
    ) -> std::fmt::Result {
        repetitive! {
            match param_id {
                __ if __ == Some(Params::VOLUME.id) => write!(writer, "{value:+.2}dB"),
                @for p in ['attack_duration, 'decay_duration, 'release_duration] {
                    __ if __ == Some(Params::ADSR.@p.id) => write!(writer, "{value:+.2}s"),
                }
                @for p in ['attack_power, 'decay_power, 'sustain, 'release_power] {
                    __ if __ == Some(Params::ADSR.@p.id) => write!(writer, "{value:+.2}"),
                }
                _ => Err(std::fmt::Error),
            }
        }
    }

    fn text_to_value(&mut self, _param_id: ClapId, text: &CStr) -> Option<f64> {
        let text = text.to_str().unwrap();
        // match param_id {
        //     __ if __ == Some(Params::VOLUME.id) => f64::from_str(&text).ok(),
        //     _ => None,
        // }
        f64::from_str(&text).ok()
    }

    fn flush(
        &mut self,
        input_parameter_changes: &InputEvents,
        _output_parameter_changes: &mut OutputEvents,
    ) {
        for event in input_parameter_changes {
            if self.shared.params.handle_event(event) {
                continue;
            }
        }
    }
}

impl<'a> PluginAudioProcessorParams for SchoffhauzerSynthAudioProcessor<'a> {
    fn flush(
        &mut self,
        input_parameter_changes: &InputEvents,
        _output_parameter_changes: &mut OutputEvents,
    ) {
        for event in input_parameter_changes {
            if self.shared.params.handle_event(event) {
                continue;
            }
        }
    }
}
