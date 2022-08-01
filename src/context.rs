use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    DefaultStreamConfigError, PlayStreamError, Stream,
};
use oddio::{Handle, Mixer};
use thiserror::Error;
use tracing::error;

use crate::{Music, MusicData};

pub type MixerHandle = Handle<Mixer<[f32; 2]>>;

pub struct Context {
    mixer_handle: MixerHandle,
    #[allow(unused)]
    stream: Stream, // stream unused, but needs to be alive for the entire lifetime of Context (otherwise, no output is audible)
}

#[derive(Debug, Error)]
pub enum ContextCreationError {
    #[error("No default output device available.")]
    NoDefaultDevice,

    #[error("{0}")]
    DefaultStreamConfigError(#[from] DefaultStreamConfigError),

    #[error("{0}")]
    PlayStreamError(#[from] PlayStreamError),

    #[error("{0}")]
    BuildStreamError(#[from] cpal::BuildStreamError),
}

impl Context {
    pub fn new(mixer_handle: MixerHandle, stream: Stream) -> Self {
        Self {
            mixer_handle,
            stream,
        }
    }

    pub fn try_default() -> Result<Self, ContextCreationError> {
        // Configure cpal
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(ContextCreationError::NoDefaultDevice)?;
        let device_sample_rate = device.default_output_config()?.sample_rate();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: device_sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };

        let (mixer_handle, mixer) = oddio::split(oddio::Mixer::new());

        let stream = device.build_output_stream(
            &config,
            move |out_flat: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let out_stereo: &mut [[f32; 2]] = oddio::frame_stereo(out_flat);
                oddio::run(&mixer, device_sample_rate.0, out_stereo);
            },
            move |err| {
                error!("{}", err);
            },
        )?;
        stream.play()?;

        Ok(Self {
            mixer_handle,
            stream,
        })
    }

    pub fn mixer_handle(&self) -> &Handle<Mixer<[f32; 2]>> {
        &self.mixer_handle
    }

    pub fn mixer_handle_mut(&mut self) -> &mut Handle<Mixer<[f32; 2]>> {
        &mut self.mixer_handle
    }

    pub fn play(&mut self, data: &MusicData) -> Music {
        Music::new(self.mixer_handle.control().play(data.music()))
    }

    pub fn restart(&mut self, mut music: Music, data: &MusicData) -> Music {
        music.stop();
        self.play(data)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::try_default().unwrap()
    }
}
