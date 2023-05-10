use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct ExtraStats {
    pub proficiency_bonus: u32,
}

impl Default for ExtraStats {
    fn default() -> ExtraStats {
        ExtraStats {
            proficiency_bonus: 2
        }
    }
}