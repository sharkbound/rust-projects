use serde::{Deserialize, Serialize};

// fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
pub trait FromToJson<'de>: Serialize + Deserialize<'de> {
    fn from_json_string(string: &'de str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(string)
    }

    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}