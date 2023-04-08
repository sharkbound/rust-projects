use crate::{AttributeStats, DndCampaign, Note};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub stats: AttributeStats,
    pub note: Note,
    pub character_id: Uuid,
}

impl Character {
    pub fn new(name: &str, stats: AttributeStats) -> Self {
        Character { name: name.to_owned(), stats, note: Default::default(), character_id: Uuid::new_v4() }
    }

    pub fn with_default_stats(name: &str) -> Self {
        Character { name: name.to_owned(), stats: Default::default(), note: Default::default(), character_id: Uuid::new_v4() }
    }

    pub fn edit_note(&mut self, mut edit: impl FnMut(&mut Note)) -> &mut Self {
        edit(&mut self.note);
        self
    }
}

impl Default for Character {
    fn default() -> Self {
        Character { name: Default::default(), stats: Default::default(), note: Default::default(), character_id: Uuid::new_v4() }
    }
}