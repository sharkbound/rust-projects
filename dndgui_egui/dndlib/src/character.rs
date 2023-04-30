use crate::{AttributeStats, DndCampaign, Note, Race};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub stats: AttributeStats,
    pub note: Note,
    pub race: Race,
    pub level: u32,
    pub id: Uuid,
}

impl Character {
    pub fn new(name: &str, race: Race, stats: AttributeStats, level: Option<u32>) -> Self {
        Character {
            name: name.to_owned(),
            stats,
            note: Default::default(),
            id: Uuid::new_v4(),
            race,
            level: level.unwrap_or(1),
        }
    }

    pub fn edit(mut self, mut edit: impl FnMut(&mut Character)) -> Self {
        edit(&mut self);
        self
    }

    pub fn edit_as_ref_mut(&mut self, mut edit: impl FnMut(&mut Character)) {
        edit(self);
    }
    pub fn with_default_stats(name: &str, race: Race) -> Self {
        Character {
            name: name.to_owned(),
            stats: Default::default(),
            note: Default::default(),
            id: Uuid::new_v4(),
            race,
            level: 1,
        }
    }

    pub fn edit_note(&mut self, mut edit: impl FnMut(&mut Note)) -> &mut Self {
        edit(&mut self.note);
        self
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            race: Race::NotSet,
            name: Default::default(),
            stats: Default::default(),
            note: Default::default(),
            id: Uuid::new_v4(),
            level: 1,
        }
    }
}