use std::marker::PhantomData;

fn main() {
    
}

struct Locked;
struct Unlocked;


struct Note<State = Locked> {
    _phantom_data: PhantomData<State>,
    title: String,
    content: String,
}

impl Note<Locked> {
    fn unlock(self) -> Note<Unlocked> {
        Note {
            _phantom_data: PhantomData,
            title: self.title,
            content: self.content,
        }
    }
}

impl Note<Unlocked> {
    fn lock(self) -> Note<Locked> {
        Note {
            _phantom_data: PhantomData,
            title: self.title,
            content: self.content,
        }
    }

    fn contents(&self) -> &str {
        &self.content
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn edit_title(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }

    fn edit_content(&mut self, new_content: &str) {
        self.content = String::from(new_content);
    }
}