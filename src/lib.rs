#![deny(clippy::all)]
#![deny(rust_2018_idioms)]

mod context;
mod music;
mod sfx;

pub use context::Context;
pub use music::*;
pub use sfx::*;
