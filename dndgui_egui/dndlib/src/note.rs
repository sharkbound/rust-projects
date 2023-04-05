use crate::FromToJson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    title: String,
    content: String,
}

impl Note {
    pub fn new(title: &str, content: &str) -> Self {
        Note { title: title.to_owned(), content: content.to_owned() }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
