#[derive(Copy, Clone)]
pub struct RangeI64 {
    pub start: i64,
    pub end: i64,
    pub step: Option<i64>,
}

impl RangeI64 {
    pub fn get_step(&self) -> i64 {
        self.step.unwrap_or(1)
    }

    pub fn exclusive(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
            step: Some(1),
        }
    }

    pub fn exclusive_step(start: i64, end: i64, step: i64) -> Self {
        Self {
            start,
            end,
            step: Some(step),
        }
    }
}

#[derive(Copy, Clone)]
pub struct RangeI64Iterator {
    range: RangeI64,
    current: i64,
}

impl Iterator for RangeI64Iterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.range.start < self.range.end) && self.current < self.range.end {
            self.current += self.range.get_step();
            return Some(self.current);
        } else if (self.range.start > self.range.end) && self.current > self.range.end {
            self.current -= self.range.get_step();
            return Some(self.current);
        }

        return None;
    }
}

impl IntoIterator for RangeI64 {
    type Item = i64;
    type IntoIter = RangeI64Iterator;

    fn into_iter(self) -> Self::IntoIter {
        RangeI64Iterator { range: self.into(), current: self.start }
    }
}


fn main() {
    let range = RangeI64::exclusive(10, -10);
    let range_evens = range.into_iter().filter(|x| x & 1 == 0);
    for x in range_evens {
        println!("{}", x)
    }
}
