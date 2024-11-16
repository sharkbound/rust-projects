use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data: Rc<RefCell<Box<dyn Data>>> = Rc::new(RefCell::new(Box::new(ItemData::new())));
    let mut items = [Item::new(data.clone()), Item::new(data.clone())];
    items[0].push("Hello");
    items[1].push("World");
    items[0].data.borrow().print();
}

struct Item {
    data: Rc<RefCell<Box<dyn Data>>>,
}

impl Item {
    fn new(data: Rc<RefCell<Box<dyn Data>>>) -> Self {
        Item { data }
    }

    fn push(&mut self, data: &str) {
        self.data.borrow_mut().add(data);
    }
}

trait Data {
    fn add(&mut self, data: &str);
    fn print(&self);
}

struct ItemData {
    pub data: Vec<String>,
}

impl ItemData {
    fn new() -> Self {
        ItemData { data: Vec::new() }
    }
}

impl Data for ItemData {
    fn add(&mut self, data: &str) {
        self.data.push(data.to_string());
    }
    fn print(&self) {
        println!("{:?}", self.data);
    }
}
