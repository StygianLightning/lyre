use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use oddio::{Handle, Mixer};
use tracing::error;

use crate::{music::MusicHandle, Music, MusicData};

pub type MixerHandle = Handle<Mixer<[f32; 2]>>;

pub struct Context {
    mixer_handle: MixerHandle,
    #[allow(unused)]
    stream: Stream, // stream unused, but needs to be alive for the entire lifetime of Context (otherwise, no output is audible)
}

impl Context {
    pub fn mixer_handle(&self) -> &Handle<Mixer<[f32; 2]>> {
        &self.mixer_handle
    }

    pub fn mixer_handle_mut(&mut self) -> &mut Handle<Mixer<[f32; 2]>> {
        &mut self.mixer_handle
    }

    pub fn play(&mut self, data: &MusicData) -> MusicHandle {
        self.mixer_handle.control().play(data.music())
    }

    pub fn restart(&mut self, music: &mut Music, data: &MusicData) -> MusicHandle {
        music.stop();
        self.play(data)
    }
}

impl Default for Context {
    fn default() -> Self {
        // Configure cpal
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device available.");
        let device_sample_rate = device.default_output_config().unwrap().sample_rate();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: device_sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };

        let (mixer_handle, mixer) = oddio::split(oddio::Mixer::new());

        let stream = device
            .build_output_stream(
                &config,
                move |out_flat: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let out_stereo: &mut [[f32; 2]] = oddio::frame_stereo(out_flat);
                    oddio::run(&mixer, device_sample_rate.0, out_stereo);
                },
                move |err| {
                    error!("{}", err);
                },
            )
            .unwrap();
        stream.play().unwrap();

        Self {
            mixer_handle,
            stream,
        }
    }
}
