use std::collections::VecDeque;

trait IO {
    fn next(&mut self) -> Option<i64>;
    fn add(&mut self, value: i64);
}

struct FIFOIO {
    items: VecDeque<i64>,
}

impl IO for FIFOIO {
    fn next(&mut self) -> Option<i64> {
        self.items.pop_front()
    }

    fn add(&mut self, value: i64) {
        self.items.push_back(value);
    }
}

struct LIFOIO {
    items: VecDeque<i64>,
}

impl IO for LIFOIO {
    fn next(&mut self) -> Option<i64> {
        self.items.pop_back()
    }
    fn add(&mut self, value: i64) {
        self.items.push_back(value);
    }
}
