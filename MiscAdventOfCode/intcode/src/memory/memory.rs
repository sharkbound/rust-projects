use rustc_hash::FxHashMap;

pub struct Memory {
    memory: FxHashMap<usize, u64>,
}

impl Memory {
    pub fn empty() -> Self {
        Memory {
            memory: FxHashMap::default(),
        }
    }
    pub fn get_direct(&self, index: usize) -> Option<u64> {
        self.memory.get(&index).cloned()
    }

    pub fn set(&mut self, index: usize, value: u64) {
        self.memory.insert(index, value);
    }
}
