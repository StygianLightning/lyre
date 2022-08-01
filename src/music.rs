use std::sync::Arc;

use oddio::{Cycle, Frames, Gain, Handle, Stop};

pub type MusicFrames = Arc<Frames<[f32; 2]>>;
pub type MusicContent = Gain<Cycle<[f32; 2]>>;
pub type MusicHandle = Handle<Stop<MusicContent>>;

#[derive(Debug)]
pub struct MusicData {
    pub frames: MusicFrames,
}

impl MusicData {
    pub(crate) fn music(&self) -> MusicContent {
        Gain::new(Cycle::new(Arc::clone(&self.frames)), 1.0)
    }
}

impl From<MusicFrames> for MusicData {
    fn from(frames: MusicFrames) -> Self {
        Self { frames }
    }
}

pub struct Music {
    handle: MusicHandle,
}

impl Music {
    pub fn new(handle: MusicHandle) -> Self {
        Self { handle }
    }

    pub fn is_paused(&mut self) -> bool {
        self.handle.control::<Stop<_>, _>().is_paused()
    }

    pub fn pause(&mut self) {
        self.handle.control::<Stop<_>, _>().pause();
    }

    pub fn resume(&mut self) {
        self.handle.control::<Stop<_>, _>().resume();
    }

    pub fn stop(&mut self) {
        self.handle.control::<Stop<_>, _>().stop();
    }

    pub fn gain(&mut self) -> f32 {
        self.handle.control::<Gain<_>, _>().gain()
    }

    pub fn set_gain(&mut self, value: f32) {
        self.handle.control::<Gain<_>, _>().set_gain(value);
    }
}
