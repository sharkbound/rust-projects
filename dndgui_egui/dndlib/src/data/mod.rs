use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CampaignData {
    #[serde(skip)]
    open_character_windows: Vec<Uuid>,
}

impl CampaignData {
    pub fn add_open_character_window(&mut self, character_id: Uuid) {
        if !self.open_character_windows.contains(&character_id) {
            self.open_character_windows.push(character_id);
        }
    }

    pub fn open_character_windows(&self) -> &[Uuid] {
        &self.open_character_windows
    }

    pub fn remove_open_character_window(&mut self, character_id: Uuid) {
        self.open_character_windows.retain(|&x| x != character_id);
    }
}

impl Default for CampaignData {
    fn default() -> Self {
        Self {
            open_character_windows: vec![]
        }
    }
}