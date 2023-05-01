use std::slice::Iter;
use crate::{CampaignData, Character, CharacterInfo, Note};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct DndCampaign {
    title: String,
    characters: Vec<Character>,
    global_notes: Vec<Note>,
    data: CampaignData,
}

impl DndCampaign {
    pub fn new(title: &str, characters: Vec<Character>, notes: Vec<Note>) -> Self {
        Self {
            characters,
            title: title.into(),
            global_notes: notes,
            data: Default::default(),
        }
    }

    pub fn title(&self) -> &str { &self.title }
    pub fn data_mut(&mut self) -> &mut CampaignData { &mut self.data }
    pub fn data(&self) -> &CampaignData { &self.data }
    pub fn character_ids(&self) -> Vec<Uuid> { self.iter_characters().map(|c| c.id).collect() }
    pub fn all_character_infos(&self) -> Vec<CharacterInfo> { self.characters.iter().map(|c| c.as_info()).collect() }

    pub fn iter_characters(&self) -> Iter<'_, Character> { self.characters.iter() }
    pub fn iter_notes(&self) -> CampaignNotesIter { CampaignNotesIter { global_note_index: 0, character_index: 0, campaign: self } }

    pub fn add_character(&mut self, character: Character) {
        self.characters.push(character);
    }

    pub fn add_global_note(&mut self, note: Note) {
        self.global_notes.push(note);
    }

    pub fn find_character(&self, search: impl FnMut(&&Character) -> bool) -> Option<&Character> {
        self.characters.iter().find(search)
    }

    pub fn find_character_by_id(&self, id: Uuid) -> Option<&Character> {
        self.find_character(|c| c.id == id)
    }

    pub fn find_character_info(&self, search: impl FnMut(&&Character) -> bool) -> Option<CharacterInfo> {
        self.find_character(search).map(|c| c.as_info())
    }

    pub fn find_character_info_by_id(&self, id: Uuid) -> Option<CharacterInfo> {
        self.find_character_info(|c| c.id == id)
    }
}

impl Default for DndCampaign {
    fn default() -> Self {
        Self::new("", vec![], vec![])
    }
}

#[derive(Debug)]
pub struct NoteIterItem<'a> {
    note: &'a Note,
    character: Option<CharacterInfo>,
}

pub struct CampaignNotesIter<'a> {
    campaign: &'a DndCampaign,
    character_index: usize,
    global_note_index: usize,
}

impl<'a> Iterator for CampaignNotesIter<'a> {
    type Item = NoteIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.campaign.global_notes.get(self.global_note_index) {
            Some(note) => {
                self.global_note_index += 1;
                Some(NoteIterItem {
                    note,
                    character: None,
                })
            }
            None => {
                match self.campaign.characters.get(self.character_index) {
                    Some(character) => {
                        self.character_index += 1;
                        Some(NoteIterItem {
                            note: &character.note,
                            character: Some(character.as_info()),
                        })
                    }
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod note_iterator_test {
    use crate::Race;
    use super::*;

    #[test]
    fn test_note_iterator() {
        let campaign = DndCampaign::new("", vec![
            {
                let mut c = Character::with_default_stats("james", Race::NotSet);
                c.edit_note(|note| {
                    note.edit_content(|_| "james' character note!".into());
                });
                c
            },
            {
                let mut c = Character::with_default_stats("nick", Race::Human);
                c.edit_note(|note| {
                    note.edit_content(|_| "nick's character note!".into());
                });
                c
            },
        ], vec![Note::with_empty_title("note #1 here!"), Note::with_empty_title("note #2 here!")]);

        let notes = campaign.iter_notes().collect::<Vec<_>>();
        assert_eq!(notes.len(), 4);

        assert!(notes[0].character.is_none());
        assert!(notes[1].character.is_none());
        assert!(notes[2].character.is_some());
        assert!(notes[3].character.is_some());
    }
}