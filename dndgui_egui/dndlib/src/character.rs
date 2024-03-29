use crate::{AbilityScores, ExtraStats, HelperExt, Note, Race, SavingThrowBonuses, SavingThrowProficiencies, SavingThrowsModifiers, SkillBonuses, SkillModifiers, SkillProficiencies};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub ability_scores: AbilityScores,
    pub note: Note,
    pub race: Race,
    pub level: u32,
    pub id: Uuid,
    pub extra_stats: ExtraStats,
    // TODO: stores bonus stats here? maybe?
}

impl Character {
    pub fn new(name: &str, race: Race, ability_scores: AbilityScores, level: Option<u32>) -> Self {
        Character {
            name: name.to_owned(),
            ability_scores,
            note: Default::default(),
            id: Uuid::new_v4(),
            race,
            level: level.unwrap_or(1),
            ..Default::default()
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
            ability_scores: Default::default(),
            note: Default::default(),
            id: Uuid::new_v4(),
            race,
            level: 1,
            ..Default::default()
        }
    }

    pub fn as_info(&self) -> CharacterInfo {
        CharacterInfo {
            name: self.name.clone(),
            ability_scores: self.ability_scores,
            race: self.race.clone(),
            level: self.level,
            id: self.id,
            skill_modifiers: self.ability_scores.skill_modifiers(
                Some(SkillBonuses::default().apply_own(|mut it| {
                    it.proficiency_bonus = self.extra_stats.proficiency_bonus;
                    it
                })),
                Some(SkillProficiencies::default()),
            ),
            extra_stats: self.extra_stats,
            saving_throws: self.ability_scores.saving_throw_modifiers(
                Some(SavingThrowBonuses::default().apply_own(|mut it| {
                    it.proficiency_bonus = self.extra_stats.proficiency_bonus;
                    it
                })),
                Some(SavingThrowProficiencies::default()),
            ),
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
            ability_scores: Default::default(),
            note: Default::default(),
            id: Uuid::new_v4(),
            level: 1,
            extra_stats: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterInfo {
    pub name: String,
    pub id: Uuid,
    pub race: Race,
    pub ability_scores: AbilityScores,
    pub level: u32,
    pub extra_stats: ExtraStats,
    pub skill_modifiers: SkillModifiers,
    pub saving_throws: SavingThrowsModifiers,
    // TODO: stores bonus stats here? maybe?
}