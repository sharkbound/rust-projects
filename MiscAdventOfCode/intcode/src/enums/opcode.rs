pub const OPCODE_ADD_CODE: u32 = 1;
pub const OPCODE_SUB_CODE: u32 = 2;
pub const OPCODE_STOP_CODE: u32 = 99;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OPCode {
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
impl OPCode {
    pub fn add(addr_add_1: u32, addr_add_2: u32, addr_result: u32) -> Self {
        OPCode::ADD {
            addr_add_1: addr_add_1 as usize,
            addr_add_2: addr_add_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn sub(addr_sub_1: u32, addr_sub_2: u32, addr_result: u32) -> Self {
        OPCode::SUB {
            addr_sub_1: addr_sub_1 as usize,
            addr_sub_2: addr_sub_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn stop() -> Self {
        OPCode::STOP
    }

    pub fn code(&self) -> u32 {
        match self {
            OPCode::ADD { .. } => 1,
            OPCode::SUB { .. } => 2,
            OPCode::STOP => 99,
        }
    }

    pub fn opcode_len(&self) -> u32 {
        opcode_param_len(self.code())
    }
}

pub fn opcode_param_len(code: u32) -> u32 {
    match code {
        1 => 3,
        2 => 3,
        99 => 0,
        _ => panic!("Invalid opcode: {}", code),
    }
}
