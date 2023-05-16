use serde::{Deserialize, Serialize};
use crate::{AbilityScoreModifiers, ProficiencyStatus};

pub struct SavingThrowsModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl SavingThrowsModifiers {
    pub fn from_ability_score_modifiers(ability_score_modifiers: AbilityScoreModifiers, bonuses: Option<SavingThrowBonuses>, proficiencies: Option<SavingThrowProficiencies>) -> Self {
        let proficiencies = proficiencies.unwrap_or_default();
        let bonuses = bonuses.unwrap_or_default();
        let calc_bonus = |modifier: i32, proficiency_status: ProficiencyStatus| {
            modifier + bonuses.strength + (bonuses.proficiency_bonus as i32 * proficiency_status.proficiency_multiplier().min(1))
        };

        Self {
            strength: calc_bonus(ability_score_modifiers.strength, proficiencies.strength),
            dexterity: calc_bonus(ability_score_modifiers.dexterity, proficiencies.dexterity),
            constitution: calc_bonus(ability_score_modifiers.constitution, proficiencies.constitution),
            intelligence: calc_bonus(ability_score_modifiers.intelligence, proficiencies.intelligence),
            wisdom: calc_bonus(ability_score_modifiers.wisdom, proficiencies.wisdom),
            charisma: calc_bonus(ability_score_modifiers.charisma, proficiencies.charisma),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SavingThrowBonuses {
    pub proficiency_bonus: u32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}


#[derive(Serialize, Deserialize, Copy, Clone, Default)]
pub struct SavingThrowProficiencies {
    pub strength: ProficiencyStatus,
    pub dexterity: ProficiencyStatus,
    pub constitution: ProficiencyStatus,
    pub intelligence: ProficiencyStatus,
    pub wisdom: ProficiencyStatus,
    pub charisma: ProficiencyStatus,
}


#[cfg(test)]
mod saving_throws_modifiers_tests {
    use crate::{AbilityScores, HelperExt};
    use super::*;

    #[test]
    fn bonuses_from_ability_scores() {
        let bonuses = SavingThrowBonuses::default().apply_own(|mut it| {
            it.proficiency_bonus = 2;
            it
        });

        let ability_score_modifiers = AbilityScoreModifiers::from_ability_scores(AbilityScores {
            strength: 12,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        });

        let proficiencies = SavingThrowProficiencies::default().apply_own(|mut it| {
            it.strength = ProficiencyStatus::Full;
            it
        });


        let bonuses = SavingThrowsModifiers::from_ability_score_modifiers(ability_score_modifiers, Some(bonuses), Some(proficiencies));
        assert_eq!(bonuses.strength, 3);
    }
}