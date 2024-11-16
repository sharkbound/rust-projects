pub const OPCODE_ADD_CODE: i64 = 1;
pub const OPCODE_SUB_CODE: i64 = 2;
pub const OPCODE_STOP_CODE: i64 = 99;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OPCode {
    Add {
        addr_add_1: usize,
        addr_add_2: usize,
        addr_result: usize,
    },
    Sub {
        addr_sub_1: usize,
        addr_sub_2: usize,
        addr_result: usize,
    },
    Stop,
}
impl OPCode {
    pub fn add(addr_add_1: u32, addr_add_2: u32, addr_result: u32) -> Self {
        OPCode::Add {
            addr_add_1: addr_add_1 as usize,
            addr_add_2: addr_add_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn sub(addr_sub_1: u32, addr_sub_2: u32, addr_result: u32) -> Self {
        OPCode::Sub {
            addr_sub_1: addr_sub_1 as usize,
            addr_sub_2: addr_sub_2 as usize,
            addr_result: addr_result as usize,
        }
    }

    pub fn stop() -> Self {
        OPCode::Stop
    }

    pub fn code(&self) -> i64 {
        match self {
            OPCode::Add { .. } => 1,
            OPCode::Sub { .. } => 2,
            OPCode::Stop => 99,
        }
    }

    pub fn opcode_len(&self) -> u32 {
        opcode_param_len(self.code())
    }
}

pub fn opcode_param_len(code: i64) -> u32 {
    match code {
        1 => 3,
        2 => 3,
        99 => 0,
        _ => panic!("Invalid opcode: {}", code),
    }
}
