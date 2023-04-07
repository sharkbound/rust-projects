use crate::{AttributeStats, DndCampaign, Note};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: String,
    stats: AttributeStats,
    note: Note,
    character_id: Uuid,
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

    pub fn name(&self) -> &str { &self.name }
    pub fn stats(&self) -> &AttributeStats { &self.stats }
    pub fn note(&self) -> &Note { &self.note }
}
