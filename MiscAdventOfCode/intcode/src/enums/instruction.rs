#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    ADD {
        addr_add_1: usize,
        addr_add_2: usize,
        addr_result: usize,
    },
    SUB {
        addr_sub_1: usize,
        addr_sub_2: usize,
        addr_result: usize,
    },
    STOP,
}
impl Instruction {
    pub fn add(addr_add_1: u32, addr_add_2: u32, addr_result: u32) -> Self {
        Instruction::ADD {
            addr_add_1: addr_add_1 as usize,
            addr_add_2: addr_add_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn sub(addr_sub_1: u32, addr_sub_2: u32, addr_result: u32) -> Self {
        Instruction::SUB {
            addr_sub_1: addr_sub_1 as usize,
            addr_sub_2: addr_sub_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn stop() -> Self {
        Instruction::STOP
    }

    pub fn code(&self) -> u32 {
        match self {
            Instruction::ADD { .. } => 1,
            Instruction::SUB { .. } => 2,
            Instruction::STOP => 99,
        }
    }
}
