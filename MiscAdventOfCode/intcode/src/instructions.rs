use crate::opcode::opcode_param_len;
use crate::{opcode, OPCode};

pub struct Instructions {
    raw_instructions: Vec<u32>,
}

impl Instructions {
    pub fn new(raw_instructions: Vec<u32>) -> Self {
        Instructions { raw_instructions }
    }

    pub fn read_opcode(&self, index: usize) -> Option<OPCode> {
        let opcode_code = self.raw_instructions.get(index).cloned()?;
        let opcode_len = opcode_param_len(opcode_code);
        let params = &self.raw_instructions[index + 1..=index + opcode_len as usize];
        let get_param = |i| params.get(i).cloned().map(|x| x as usize);

        Some(match opcode_code {
            opcode::OPCODE_ADD_CODE => OPCode::ADD {
                addr_add_1: get_param(0)?,
                addr_add_2: get_param(1)?,
                addr_result: get_param(2)?,
            },
            opcode::OPCODE_SUB_CODE => OPCode::SUB {
                addr_sub_1: get_param(0)?,
                addr_sub_2: get_param(1)?,
                addr_result: get_param(2)?,
            },
            opcode::OPCODE_STOP_CODE => OPCode::STOP,
            _ => return None,
        })
    }
}
