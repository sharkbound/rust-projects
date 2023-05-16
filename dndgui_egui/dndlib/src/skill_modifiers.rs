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
    fn calc_bonus(ability_score: u32, bonus: i32, proficiency_bonus: u32, proficiency: ProficiencyStatus) -> i32 {
        ability_score as i32 + bonus + (proficiency_bonus as i32 * proficiency.proficiency_multiplier())
    }

    pub fn from_ability_scores(ability_scores: AbilityScores, bonuses: Option<SkillBonuses>, proficiencies: Option<SkillProficiencies>) -> Self {
        let proficiencies = proficiencies.unwrap_or_default();
        let bonuses = bonuses.unwrap_or_default();
        let proficiency_bonus = bonuses.proficiency_bonus;

        Self {
            acrobatics: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.dexterity, bonuses.acrobatics, proficiency_bonus, proficiencies.acrobatics),
                proficiencies.acrobatics,
            ),
            animal_handling: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.wisdom, bonuses.animal_handling, proficiency_bonus, proficiencies.animal_handling),
                proficiencies.animal_handling,
            ),
            arcana: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.intelligence, bonuses.arcana, proficiency_bonus, proficiencies.arcana),
                proficiencies.arcana,
            ),
            athletics: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.strength, bonuses.athletics, proficiency_bonus, proficiencies.athletics),
                proficiencies.athletics,
            ),
            deception: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.charisma, bonuses.deception, proficiency_bonus, proficiencies.deception),
                proficiencies.deception,
            ),
            history: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.intelligence, bonuses.history, proficiency_bonus, proficiencies.history),
                proficiencies.history,
            ),
            insight: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.wisdom, bonuses.insight, proficiency_bonus, proficiencies.insight),
                proficiencies.insight,
            ),
            intimidation: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.charisma, bonuses.intimidation, proficiency_bonus, proficiencies.intimidation),
                proficiencies.intimidation,
            ),
            investigation: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.intelligence, bonuses.investigation, proficiency_bonus, proficiencies.investigation),
                proficiencies.investigation,
            ),
            medicine: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.wisdom, bonuses.medicine, proficiency_bonus, proficiencies.medicine),
                proficiencies.medicine,
            ),
            nature: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.intelligence, bonuses.nature, proficiency_bonus, proficiencies.nature),
                proficiencies.nature,
            ),
            perception: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.wisdom, bonuses.perception, proficiency_bonus, proficiencies.perception),
                proficiencies.perception,
            ),
            performance: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.charisma, bonuses.performance, proficiency_bonus, proficiencies.performance),
                proficiencies.performance,
            ),
            persuasion: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.charisma, bonuses.persuasion, proficiency_bonus, proficiencies.persuasion),
                proficiencies.persuasion,
            ),
            religion: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.intelligence, bonuses.religion, proficiency_bonus, proficiencies.religion),
                proficiencies.religion,
            ),
            slight_of_hand: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.dexterity, bonuses.slight_of_hand, proficiency_bonus, proficiencies.slight_of_hand),
                proficiencies.slight_of_hand,
            ),
            stealth: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.dexterity, bonuses.stealth, proficiency_bonus, proficiencies.stealth),
                proficiencies.stealth,
            ),
            survival: SkillModifierStat::new(
                Self::calc_bonus(ability_scores.wisdom, bonuses.survival, proficiency_bonus, proficiencies.survival),
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
