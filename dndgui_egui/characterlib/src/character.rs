use crate::AttributeStats;
use crate::json_serialization_trait::FromToJson;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: String,
    stats: AttributeStats,
}

impl Character {
    pub fn new(name: &str, stats: AttributeStats) -> Self {
        Character { name: name.to_owned(), stats }
    }

    pub fn with_default_stats(name: &str) -> Self {
        Character { name: name.to_owned(), stats: AttributeStats::default() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stats(&self) -> &AttributeStats {
        &self.stats
    }

    pub fn as_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json_string(json: &str) -> Character {
        serde_json::from_str(json).unwrap()
    }
}

impl<'de> FromToJson<'de> for Character {}