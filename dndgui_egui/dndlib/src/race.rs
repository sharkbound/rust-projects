use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Race {
    NotSet,
    Custom(String),
    HalfElf,
    DragonBorn,
    Dwarf,
    Elf,
    Gnome,
    Halfling,
    HalfOrc,
    Orc,
    Human,
    Tiefling,
    BugBear,
    LizardFolk,
    Kobold,
}

impl Default for Race {
    fn default() -> Self {
        Race::NotSet
    }
}

impl ToString for Race {
    fn to_string(&self) -> String {
        match self {
            Race::Custom(race) => race.clone(),
            Race::HalfElf => "Half-Elf".to_owned(),
            Race::DragonBorn => "Dragon-Born".to_owned(),
            Race::Dwarf => "Dwarf".to_owned(),
            Race::Elf => "Elf".to_owned(),
            Race::Gnome => "Gnome".to_owned(),
            Race::Halfling => "Half-ling".to_owned(),
            Race::HalfOrc => "Half-Orc".to_owned(),
            Race::Orc => "Orc".to_owned(),
            Race::Human => "Human".to_owned(),
            Race::Tiefling => "Tiefling".to_owned(),
            Race::BugBear => "Bug-Bear".to_owned(),
            Race::LizardFolk => "Lizard-Folk".to_owned(),
            Race::Kobold => "Kobold".to_owned(),
            Race::NotSet => "{RACE NOT SET}".to_owned(),
        }
    }
}

impl Race {
    fn is_custom(&self) -> bool {
        matches!(self, Race::Custom(_))
    }
}