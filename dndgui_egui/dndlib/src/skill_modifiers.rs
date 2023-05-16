use serde::{Deserialize, Serialize};
use crate::{AbilityScoreModifiers};

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

    pub fn proficiency_multiplier(&self) -> i32 {
        match self {
            ProficiencyStatus::Expertise => 2,
            ProficiencyStatus::Full => 1,
            ProficiencyStatus::Half => 0,
        }
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
    pub fn new(score: i32, proficiency: ProficiencyStatus) -> Self {
        SkillModifierStat {
            proficiency,
            score,
        }
    }

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
    pub stealth: SkillModifierStat,
    pub survival: SkillModifierStat,
}

impl SkillModifiers {
    pub fn from_ability_score_modifiers(ability_score_modifiers: AbilityScoreModifiers, bonuses: Option<SkillBonuses>, proficiencies: Option<SkillProficiencies>) -> Self {
        let proficiencies = proficiencies.unwrap_or_default();
        let bonuses = bonuses.unwrap_or_default();
        let proficiency_bonus = bonuses.proficiency_bonus;
        let calc_bonus = |ability_score_modifier: i32, bonus: i32, proficiency: ProficiencyStatus| {
            ability_score_modifier + bonus + (proficiency_bonus as i32 * proficiency.proficiency_multiplier())
        };

        Self {
            acrobatics: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.dexterity, bonuses.acrobatics, proficiencies.acrobatics),
                proficiencies.acrobatics,
            ),
            animal_handling: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.wisdom, bonuses.animal_handling, proficiencies.animal_handling),
                proficiencies.animal_handling,
            ),
            arcana: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.intelligence, bonuses.arcana, proficiencies.arcana),
                proficiencies.arcana,
            ),
            athletics: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.strength, bonuses.athletics, proficiencies.athletics),
                proficiencies.athletics,
            ),
            deception: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.charisma, bonuses.deception, proficiencies.deception),
                proficiencies.deception,
            ),
            history: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.intelligence, bonuses.history, proficiencies.history),
                proficiencies.history,
            ),
            insight: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.wisdom, bonuses.insight, proficiencies.insight),
                proficiencies.insight,
            ),
            intimidation: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.charisma, bonuses.intimidation, proficiencies.intimidation),
                proficiencies.intimidation,
            ),
            investigation: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.intelligence, bonuses.investigation, proficiencies.investigation),
                proficiencies.investigation,
            ),
            medicine: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.wisdom, bonuses.medicine, proficiencies.medicine),
                proficiencies.medicine,
            ),
            nature: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.intelligence, bonuses.nature, proficiencies.nature),
                proficiencies.nature,
            ),
            perception: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.wisdom, bonuses.perception, proficiencies.perception),
                proficiencies.perception,
            ),
            performance: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.charisma, bonuses.performance, proficiencies.performance),
                proficiencies.performance,
            ),
            persuasion: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.charisma, bonuses.persuasion, proficiencies.persuasion),
                proficiencies.persuasion,
            ),
            religion: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.intelligence, bonuses.religion, proficiencies.religion),
                proficiencies.religion,
            ),
            slight_of_hand: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.dexterity, bonuses.slight_of_hand, proficiencies.slight_of_hand),
                proficiencies.slight_of_hand,
            ),
            stealth: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.dexterity, bonuses.stealth, proficiencies.stealth),
                proficiencies.stealth,
            ),
            survival: SkillModifierStat::new(
                calc_bonus(ability_score_modifiers.wisdom, bonuses.survival, proficiencies.survival),
                proficiencies.survival,
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct SkillBonuses {
    pub proficiency_bonus: u32,
    pub acrobatics: i32,
    pub animal_handling: i32,
    pub arcana: i32,
    pub athletics: i32,
    pub deception: i32,
    pub history: i32,
    pub insight: i32,
    pub intimidation: i32,
    pub investigation: i32,
    pub medicine: i32,
    pub nature: i32,
    pub perception: i32,
    pub performance: i32,
    pub persuasion: i32,
    pub religion: i32,
    pub slight_of_hand: i32,
    pub stealth: i32,
    pub survival: i32,
}


#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct SkillProficiencies {
    pub acrobatics: ProficiencyStatus,
    pub animal_handling: ProficiencyStatus,
    pub arcana: ProficiencyStatus,
    pub athletics: ProficiencyStatus,
    pub deception: ProficiencyStatus,
    pub history: ProficiencyStatus,
    pub insight: ProficiencyStatus,
    pub intimidation: ProficiencyStatus,
    pub investigation: ProficiencyStatus,
    pub medicine: ProficiencyStatus,
    pub nature: ProficiencyStatus,
    pub perception: ProficiencyStatus,
    pub performance: ProficiencyStatus,
    pub persuasion: ProficiencyStatus,
    pub religion: ProficiencyStatus,
    pub slight_of_hand: ProficiencyStatus,
    pub stealth: ProficiencyStatus,
    pub survival: ProficiencyStatus,
}


#[cfg(test)]
mod test_skill_modifiers {
    use crate::{AbilityScores, HelperExt};
    use super::*;

    #[test]
    fn test_skill_bonuses() {
        let skills = SkillModifiers::from_ability_score_modifiers(
            AbilityScores {
                dexterity: 12,
                wisdom: 0,
                intelligence: 0,
                strength: 0,
                charisma: 0,
                constitution: 0,
            }.ability_score_modifiers(),
            Some(SkillBonuses::default().apply_own(|mut it| {
                it.proficiency_bonus = 2;
                it.acrobatics = 2;
                it
            })),
            Some(SkillProficiencies::default().apply_own(|mut it| {
                it.acrobatics = ProficiencyStatus::Expertise;
                it
            })),
        );
        assert_eq!(skills.acrobatics.score, 7);

        let skills = SkillModifiers::from_ability_score_modifiers(
            AbilityScores {
                dexterity: 14,
                wisdom: 0,
                intelligence: 0,
                strength: 0,
                charisma: 0,
                constitution: 0,
            }.ability_score_modifiers(),
            Some(SkillBonuses::default().apply_own(|mut it| {
                it.proficiency_bonus = 2;
                it
            })),
            Some(SkillProficiencies::default().apply_own(|mut it| {
                it.acrobatics = ProficiencyStatus::Full;
                it
            })),
        );
        assert_eq!(skills.acrobatics.score, 4);
    }
}