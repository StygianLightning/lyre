#![deny(clippy::all)]
#![deny(rust_2018_idioms)]

mod audio_repository;
mod context;
mod music;

pub use audio_repository::*;
pub use context::Context;
pub use music::*;
