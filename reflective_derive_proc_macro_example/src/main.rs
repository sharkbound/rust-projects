use reflective_derive_proc_macro_example::Reflective;

fn main() {
    let mut counter = Counter::new("counter");
    counter.insert(32);
    counter.insert(12);
    counter.insert(24);
    println!("Average: {:?}, Total: {:?}", counter.average(), counter.total());
    println!("Struct Name: {:?}, Struct Members: {:?}", counter.name(), counter.field_names());
}

#[derive(Reflective)]
struct Counter {
    name: String,
    total: i64,
    average: f64,
    values: Vec<i64>,
}

impl Counter {
    pub fn new(name: &str) -> Counter {
        Counter {
            name: name.to_string(),
            total: 0,
            average: 0.0,
            values: Vec::new(),
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn insert(&mut self, value: i64) {
        self.total += value;
        self.values.push(value);
        self.average = self.total as f64 / self.values.len() as f64;
    }
}
