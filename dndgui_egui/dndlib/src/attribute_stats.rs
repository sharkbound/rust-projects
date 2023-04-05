use serde::{Serialize, Deserialize};
use crate::json_serialization_trait::FromToJson;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AttributeStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl AttributeStats {
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

    pub fn strength_mod(&self) -> i32 { (self.strength as i32 - 10) / 2 }
    pub fn dexterity_mod(&self) -> i32 { (self.dexterity as i32 - 10) / 2 }
    pub fn constitution_mod(&self) -> i32 { (self.constitution as i32 - 10) / 2 }
    pub fn intelligence_mod(&self) -> i32 { (self.intelligence as i32 - 10) / 2 }
    pub fn wisdom_mod(&self) -> i32 { (self.wisdom as i32 - 10) / 2 }
    pub fn charisma_mod(&self) -> i32 { (self.charisma as i32 - 10) / 2 }
}

impl Default for AttributeStats {
    fn default() -> Self {
        AttributeStats {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }
}


#[cfg(test)]
mod character_stat_tests {
    use super::*;

    #[test]
    fn test_strength_8_mod_is_negative_1() {
        let mut stats = AttributeStats::default();
        stats.strength = 8;
        assert_eq!(stats.strength_mod(), -1);
    }

    #[test]
    fn test_strength_12_mod_is_positive_1() {
        let mut stats = AttributeStats::default();
        stats.strength = 12;
        assert_eq!(stats.strength_mod(), 1);
    }
}