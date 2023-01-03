use std::fmt::{Display, Formatter};
use ::trash;

struct Item {
    id: i32,
    name: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item {{ id: {:?}, name: {:?} }}", self.id, self.name)
    }
}


impl Item {
    pub fn new(id: i32, name: &str) -> Self {
        Self { id, name: name.to_string() }
    }
}

fn main() {
    let mut trash = trash::Trash::<Item>::new();
    trash.trash(Item::new(70, "<James>"));
    trash.trash(Item::new(70, "<Not James>"));
    trash.visit(|x| println!("Visited: {}", x));
    // trash.empty();
    println!("trash has {} item in it", trash.item_count());
}
