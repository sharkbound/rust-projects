use std::collections::VecDeque;

pub trait IO {
    fn next(&mut self) -> Option<i64>;
    fn enqueue(&mut self, value: i64);
}

pub struct FIFOIO {
    items: VecDeque<i64>,
}

impl IO for FIFOIO {
    fn next(&mut self) -> Option<i64> {
        self.items.pop_front()
    }

    fn enqueue(&mut self, value: i64) {
        self.items.push_back(value);
    }
}

impl FIFOIO {
    pub fn new() -> Self {
        FIFOIO {
            items: VecDeque::new(),
        }
    }
}

pub struct LIFOIO {
    items: VecDeque<i64>,
}

impl IO for LIFOIO {
    fn next(&mut self) -> Option<i64> {
        self.items.pop_back()
    }
    fn enqueue(&mut self, value: i64) {
        self.items.push_back(value);
    }
}

impl LIFOIO {
    pub fn new() -> Self {
        LIFOIO {
            items: VecDeque::new(),
        }
    }
}
