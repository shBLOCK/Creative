use crate::SchoffhauzerSynthPluginMainThread;
use crate::utils::db::DB;
use crate::utils::modulated::Modulated;
use clack_extensions::state::PluginStateImpl;
use clack_plugin::plugin::PluginError;
use clack_plugin::stream::{InputStream, OutputStream};

#[derive_aliases::derive(..SerDe)]
struct SchoffhauzerSynthPluginState {
    volume: Modulated<DB<f32>>,
}

impl PluginStateImpl for SchoffhauzerSynthPluginMainThread<'_> {
    fn save(&mut self, output: &mut OutputStream) -> Result<(), PluginError> {
        let state = SchoffhauzerSynthPluginState {
            volume: self.shared.params.get_volume(),
        };
        serde_json::to_writer(output, &state)?;
        Ok(())
    }

    //noinspection RsUnwrap
    fn load(&mut self, input: &mut InputStream) -> Result<(), PluginError> {
        let state = serde_json::from_reader::<_, SchoffhauzerSynthPluginState>(input)?;
        *self.shared.params.volume.write().unwrap() = state.volume;
        Ok(())
    }
}
