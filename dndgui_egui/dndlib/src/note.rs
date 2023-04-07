use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    title: String,
    content: String,
    id: Uuid,
}

impl Note {
    pub fn new(title: &str, content: &str) -> Self {
        Note { title: title.to_owned(), content: content.to_owned(), id: Uuid::new_v4() }
    }

    pub fn with_empty_title(content: &str) -> Self {
        Self::new("", content)
    }

    pub fn title(&self) -> &str { &self.title }
    pub fn content(&self) -> &str { &self.content }
    pub fn id(&self) -> &Uuid { &self.id }

    pub fn edit_content(&mut self, edit: impl Fn(&String) -> String) -> &mut Self {
        self.content = edit(&self.content);
        self
    }

    pub fn edit_title(&mut self, edit: impl Fn(&String) -> String) -> &mut Self {
        self.content = edit(&self.title);
        self
    }

    pub fn clear_content(&mut self) -> &mut Self {
        self.content = String::new();
        self
    }

    pub fn clear_title(&mut self) -> &mut Self {
        self.title = String::new();
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.title = String::new();
        self.content = String::new();
        self
    }
}

impl Default for Note {
    fn default() -> Self {
        Note { title: String::new(), content: String::new(), id: Uuid::new_v4() }
    }
}