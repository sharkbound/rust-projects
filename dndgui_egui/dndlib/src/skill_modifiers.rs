use serde::{Deserialize, Serialize};
use crate::AbilityScores;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ProficiencyStatus {
    Expertise,
    Full,
    Half,
}

impl ProficiencyStatus {
    pub fn is_expertise(&self) -> bool {
        matches!(self, ProficiencyStatus::Expertise)
    }

    pub fn is_proficient(&self) -> bool {
        matches!(self, ProficiencyStatus::Full)
    }

    pub fn is_half_proficient(&self) -> bool {
        matches!(self, ProficiencyStatus::Half)
    }
}

impl Default for ProficiencyStatus {
    fn default() -> Self {
        ProficiencyStatus::Half
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SkillModifierStat {
    pub proficiency: ProficiencyStatus,
    pub score: i32,
}

impl SkillModifierStat {
    pub fn expertise(score: i32) -> Self {
        Self {
            proficiency: ProficiencyStatus::Expertise,
            score,
        }
    }

    pub fn proficient(score: i32) -> Self {
        Self {
            proficiency: ProficiencyStatus::Full,
            score,
        }
    }

    pub fn half_proficient(score: i32) -> Self {
        Self {
            proficiency: ProficiencyStatus::Half,
            score,
        }
    }
}

impl Default for SkillModifierStat {
    fn default() -> Self {
        Self {
            proficiency: ProficiencyStatus::Half,
            score: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SkillModifiers {
    pub acrobatics: SkillModifierStat,
    pub animal_handling: SkillModifierStat,
    pub arcana: SkillModifierStat,
    pub athletics: SkillModifierStat,
    pub deception: SkillModifierStat,
    pub history: SkillModifierStat,
    pub insight: SkillModifierStat,
    pub intimidation: SkillModifierStat,
    pub investigation: SkillModifierStat,
    pub medicine: SkillModifierStat,
    pub nature: SkillModifierStat,
    pub perception: SkillModifierStat,
    pub performance: SkillModifierStat,
    pub persuasion: SkillModifierStat,
    pub religion: SkillModifierStat,
    pub slight_of_hand: SkillModifierStat,
    pub survival: SkillModifierStat,
}

impl SkillModifiers {
    fn calc_bonus(score: u32) -> i32 {
        0
    }

    pub fn from(ability_scores: AbilityScores, proficiency_bonus: u32) -> Self {
        Self {
            // TODO: need to come up with a good way to store proficiencies for calculating bonuses
            ..Default::default()
        }
    }
}