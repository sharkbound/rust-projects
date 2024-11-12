use rustc_hash::FxHashMap;

pub struct Memory {
    memory: FxHashMap<usize, i64>,
}

impl Memory {
    pub fn empty() -> Self {
        Memory {
            memory: FxHashMap::default(),
        }
    }
    pub fn get_direct(&self, index: usize) -> Option<i64> {
        self.memory.get(&index).cloned()
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.memory.insert(index, value);
    }
}
