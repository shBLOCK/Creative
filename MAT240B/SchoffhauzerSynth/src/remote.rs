use crate::SchoffhauzerSynthShared;
use crate::utils::db::DB;
use crate::utils::envelope::ADSR;
use crate::utils::modulated::Modulated;
use serde::Deserialize;
use std::error::Error;
use std::ffi::CString;
use std::net::UdpSocket;
use std::sync::OnceLock;
use std::thread::JoinHandle;
use std::time::Duration;

static REMOTE_THREAD: OnceLock<JoinHandle<()>> = OnceLock::new();

pub fn spawn_remote_thread(shared: &SchoffhauzerSynthShared) {
    #[derive(Deserialize)]
    struct Packet {
        pub volume: Modulated<DB<f32>>,
        pub adsr: ADSR<Modulated<f32>>,
        pub hf_rolloff: Modulated<f32>,
    }

    let shared: &SchoffhauzerSynthShared = unsafe { std::mem::transmute::<_, &'static SchoffhauzerSynthShared<'static>>(shared) };

    shared.info(c"In spawn_remote_thread");

    REMOTE_THREAD.get_or_init(|| {
        shared.info(c"Spawning remote thread");
        std::thread::spawn(|| {
            shared.info(c"In remote thread");
            let mut buffer = Box::new([0u8; 4096]);
            loop {
                let socket = match UdpSocket::bind(("0.0.0.0", 30100)) {
                    Ok(s) => s,
                    Err(e) => {
                        shared.error(&*CString::new(format!("Unable to bind UDP socket: {e:?}")).unwrap());
                        std::thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                };

                loop {
                    || -> Result<(), Box<dyn Error>> {
                        let length = socket.recv(&mut buffer[..])?;
                        let buffer = &buffer[..length];
                        let packet = serde_json::from_slice::<Packet>(buffer)?;
                        shared.params.volume.set(packet.volume).unwrap();
                        shared.params.adsr.set(packet.adsr).unwrap();
                        shared.params.hf_rolloff.set(packet.hf_rolloff).unwrap();
                        Ok(())
                    }()
                    .unwrap_or_else(|e| {
                        shared.error(&*CString::new(format!(
                            "Failed to receive remote packet: {e:?}"
                        )).unwrap())
                    });
                }
            }
        })
    });
}
