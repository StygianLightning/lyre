#![deny(clippy::all)]
#![deny(rust_2018_idioms)]

pub mod context;
#[cfg(feature = "wav")]
pub mod wav_loader;
