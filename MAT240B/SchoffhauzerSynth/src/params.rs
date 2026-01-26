use crate::utils::db::DB;
use crate::utils::modulated::Modulated;
use crate::{SchoffhauzerSynthAudioProcessor, SchoffhauzerSynthPluginMainThread};
use clack_extensions::params::{
    ParamDisplayWriter, ParamInfo, ParamInfoFlags, ParamInfoWriter, PluginAudioProcessorParams,
    PluginMainThreadParams,
};
use clack_plugin::events::UnknownEvent;
use clack_plugin::events::event_types::{ParamModEvent, ParamValueEvent};
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::{ClapId, InputEvents, OutputEvents};
use clack_plugin::utils::Cookie;
use static_assertions::assert_impl_all;
use std::f64;
use std::ffi::CStr;
use std::fmt::Write as _;
use std::str::FromStr;
use std::sync::Mutex;

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
    pub volume: Mutex<Modulated<DB<f32>>>,
}

assert_impl_all!(SchoffhauzerSynthPluginParams: Send, Sync);

impl Default for SchoffhauzerSynthPluginParams {
    fn default() -> Self {
        Self {
            volume: Mutex::new(Modulated::new(DB(0.0), DB(0.0))),
        }
    }
}

impl SchoffhauzerSynthPluginParams {
    pub const ID_VOLUME: ClapId = ClapId::new(0);

    pub fn get_volume(&self) -> Modulated<DB<f32>> {
        *self.volume.lock().unwrap()
    }

    pub fn handle_param_value_event(&self, event: &ParamValueEvent) {
        match event.param_id() {
            Some(Self::ID_VOLUME) => self.volume.lock().unwrap().value = DB(event.value() as f32),
            _ => {}
        }
    }

    pub fn handle_param_mod_event(&self, event: &ParamModEvent) {
        match event.param_id() {
            Some(Self::ID_VOLUME) => {
                self.volume.lock().unwrap().modulation = DB(event.amount() as f32)
            }
            _ => {}
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
        1
    }

    fn get_info(&mut self, param_index: u32, info: &mut ParamInfoWriter) {
        let mut i = 0u32..;
        if param_index == i.next().unwrap() {
            info.set(&ParamInfo {
                id: ClapId::new(0),
                flags: ParamInfoFlags::IS_AUTOMATABLE_ALL | ParamInfoFlags::IS_MODULATABLE_ALL,
                cookie: Cookie::empty(),
                name: b"Volume",
                module: b"",
                min_value: -100.0,
                max_value: 100.0,
                default_value: 0.0,
            });
        }
        if param_index == i.next().unwrap() {}
    }

    fn get_value(&mut self, param_id: ClapId) -> Option<f64> {
        match param_id {
            SchoffhauzerSynthPluginParams::ID_VOLUME => {
                Some(self.shared.params.get_volume().modulated().db() as f64)
            }
            _ => None,
        }
    }

    fn value_to_text(
        &mut self,
        param_id: ClapId,
        value: f64,
        writer: &mut ParamDisplayWriter,
    ) -> std::fmt::Result {
        match param_id {
            SchoffhauzerSynthPluginParams::ID_VOLUME => write!(writer, "{value:+.2} dB"),
            _ => Err(std::fmt::Error),
        }
    }

    fn text_to_value(&mut self, param_id: ClapId, text: &CStr) -> Option<f64> {
        let text = text.to_str().unwrap();
        match param_id {
            SchoffhauzerSynthPluginParams::ID_VOLUME => f64::from_str(&text).ok(),
            _ => None,
        }
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
        output_parameter_changes: &mut OutputEvents,
    ) {
        for event in input_parameter_changes {
            if self.shared.params.handle_event(event) {
                continue;
            }
        }
    }
}
