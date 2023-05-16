use serde::{Deserialize, Serialize};
use crate::{SavingThrowBonuses, SavingThrowProficiencies, SavingThrowsModifiers, SkillBonuses, SkillModifiers, SkillProficiencies};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AbilityScores {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl AbilityScores {
    pub fn new(strength: u32, dexterity: u32, constitution: u32, intelligence: u32, wisdom: u32, charisma: u32) -> Self {
        Self {
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
        }
    }

    pub fn ability_score_modifiers(&self) -> AbilityScoreModifiers {
        AbilityScoreModifiers::from_ability_scores(*self)
    }

    pub fn skill_modifiers(&self, bonuses: Option<SkillBonuses>, proficiencies: Option<SkillProficiencies>) -> SkillModifiers {
        SkillModifiers::from_ability_score_modifiers(self.ability_score_modifiers(), bonuses, proficiencies)
    }

    pub fn saving_throw_modifiers(&self, proficiency_bonus: Option<SavingThrowBonuses>, proficiencies: Option<SavingThrowProficiencies>) -> SavingThrowsModifiers {
        SavingThrowsModifiers::from_ability_score_modifiers(self.ability_score_modifiers(), proficiency_bonus, proficiencies)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AbilityScoreModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl AbilityScoreModifiers {
    pub fn from_ability_scores(ability_scores: AbilityScores) -> Self {
        let modifier = |value| (value as i32 - 10) / 2;
        Self {
            strength: modifier(ability_scores.strength),
            dexterity: modifier(ability_scores.dexterity),
            constitution: modifier(ability_scores.constitution),
            intelligence: modifier(ability_scores.intelligence),
            wisdom: modifier(ability_scores.wisdom),
            charisma: modifier(ability_scores.charisma),
        }
    }
}

impl Default for AbilityScores {
    fn default() -> Self {
        AbilityScores {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}


#[cfg(test)]
mod character_stat_tests {
    use super::*;

    #[test]
    fn test_strength_8_mod_is_negative_1() {
        let mut stats = AbilityScores::default();
        stats.strength = 8;
        let modifiers = stats.ability_score_modifiers();
        assert_eq!(modifiers.strength, -1);
    }

    #[test]
    fn test_strength_12_mod_is_positive_1() {
        let mut stats = AbilityScores::default();
        stats.strength = 12;
        let modifiers = stats.ability_score_modifiers();
        assert_eq!(modifiers.strength, 1);
    }
}