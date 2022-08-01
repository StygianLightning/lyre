use std::collections::HashMap;

use crate::music::MusicData;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct MusicId {
    pub id: u32,
}

impl MusicId {
    pub fn new(id: u32) -> MusicId {
        Self { id }
    }
}

pub struct AudioRepository {
    music_data: HashMap<MusicId, MusicData>,
    next_music_id: MusicId,
}

impl AudioRepository {
    fn get_next_music_id(&mut self) -> MusicId {
        let ret = self.next_music_id;
        self.next_music_id = MusicId {
            id: self.next_music_id.id + 1,
        };
        ret
    }

    pub fn add_music(&mut self, data: MusicData) -> MusicId {
        let id = self.get_next_music_id();
        self.music_data.insert(id, data);
        id
    }
}

impl Default for AudioRepository {
    fn default() -> Self {
        Self {
            music_data: HashMap::new(),
            next_music_id: MusicId { id: 0 },
        }
    }
}
