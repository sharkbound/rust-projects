use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Debug)]
struct Data {
    id: u32,
    data_type: &'static str,
}

impl Data {
    pub fn new(id: u32, data_type: &'static str) -> Self {
        Self { id, data_type }
    }
}

fn main() {
    let item = Rc::new(Mutex::new(0));
    let mut event: Event<Data> = Event::new();
    let item2 = Rc::clone(&item);
    let id1 = event.add(move |x| {
        // println!("{}", x.data_type);
        let mut item = item2.lock().unwrap();
        *item += 1;
    });
    let item3 = Rc::clone(&item);
    let id2 = event.add(move |x| {
        // println!("{}", x.data_type);
        let mut item = item3.lock().unwrap();
        *item += 1;
    });
    event.call(Data::new(1337, "l33t"));
    event.remove(id1);
    event.call(Data::new(1337, "l33t"));
    println!("{:?}", item.lock());
}

struct Event<'a, T> {
    subscribers: HashMap<u32, Box<dyn Fn(&T) + 'a>>,
    last_id: u32,
}

impl<'a, T> Event<'a, T> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn add(&mut self, callback: impl Fn(&T) + 'a) -> u32 {
        self.subscribers.insert(self.last_id, Box::new(callback));
        let id = self.last_id;
        self.last_id += 1;
        id
    }

    pub fn remove(&mut self, callback_id: u32) {
        self.subscribers.remove(&callback_id);
    }

    pub fn call(&self, value: T) {
        self.subscribers.iter().for_each(move |(_, cb)| cb(&value));
    }
}