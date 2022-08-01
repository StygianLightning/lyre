use std::sync::Arc;

use oddio::{Frames, FramesSignal, Gain, Handle, Stop};

pub type SfxFrames = Arc<Frames<[f32; 2]>>;
pub type SfxContent = Gain<FramesSignal<[f32; 2]>>;
pub type SfxHandle = Handle<Stop<SfxContent>>;

#[derive(Debug)]
pub struct SfxData {
    pub frames: SfxFrames,
}

impl SfxData {
    pub(crate) fn sfx(&self) -> SfxContent {
        Gain::new(FramesSignal::new(Arc::clone(&self.frames), 0.0), 1.0)
    }
}

impl From<SfxFrames> for SfxData {
    fn from(frames: SfxFrames) -> Self {
        Self { frames }
    }
}

pub struct Sfx {
    handle: SfxHandle,
}

impl Sfx {
    pub fn new(handle: SfxHandle) -> Self {
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
