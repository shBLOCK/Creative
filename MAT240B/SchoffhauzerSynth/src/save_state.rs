use crate::SchoffhauzerSynthPluginMainThread;
use crate::utils::db::DB;
use crate::utils::modulated::Modulated;
use clack_extensions::state::PluginStateImpl;
use clack_plugin::plugin::PluginError;
use clack_plugin::stream::{InputStream, OutputStream};
use crate::utils::envelope::ADSR;

#[derive_aliases::derive(..SerDe)]
struct SchoffhauzerSynthPluginState {
    volume: DB<f32>,
    adsr: ADSR<f32>,
    hf_rolloff: f32,
}

impl PluginStateImpl for SchoffhauzerSynthPluginMainThread<'_> {
    fn save(&mut self, output: &mut OutputStream) -> Result<(), PluginError> {
        let state = SchoffhauzerSynthPluginState {
            volume: self.shared.params.get_volume().value,
            adsr: self.shared.params.get_adsr().map(|it| it.value),
            hf_rolloff: self.shared.params.get_hf_rolloff().value,
        };
        serde_json::to_writer(output, &state)?;
        Ok(())
    }

    //noinspection RsUnwrap
    fn load(&mut self, input: &mut InputStream) -> Result<(), PluginError> {
        let state = serde_json::from_reader::<_, SchoffhauzerSynthPluginState>(input)?;
        *self.shared.params.volume.write().unwrap() = Modulated::new(state.volume, DB(0.0));
        *self.shared.params.adsr.write().unwrap() = state.adsr.map(|&it| Modulated::new(it, 0.0));
        *self.shared.params.hf_rolloff.write().unwrap() = Modulated::new(state.hf_rolloff, 0.0);
        Ok(())
    }
}
