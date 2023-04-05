use crate::{Character, FromToJson, Note};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DndCampaign {
    characters: Vec<Character>,
    notes: Vec<Note>,
}

impl DndCampaign {
    pub fn new(characters: Vec<Character>, notes: Vec<Note>) -> Self {
        Self { characters, notes }
    }

    pub fn characters(&self) -> &[Character] {
        &self.characters
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn add_character(&mut self, character: Character) {
        self.characters.push(character);
    }
}

impl Default for DndCampaign {
    fn default() -> Self {
        Self::new(vec![], vec![])
    }
}
