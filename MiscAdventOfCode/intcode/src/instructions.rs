use crate::opcode::opcode_param_len;
use crate::{opcode, OPCode};

#[derive(Clone)]
pub struct Instructions {
    raw_instructions: Vec<i64>,
}

impl Instructions {
    pub fn new(raw_instructions: Vec<i64>) -> Self {
        Instructions { raw_instructions }
    }

    pub fn read_opcode(&self, index: usize) -> Option<OPCode> {
        let opcode_code = self.raw_instructions.get(index).cloned()?;
        let opcode_len = opcode_param_len(opcode_code);
        let params = &self.raw_instructions[index + 1..=index + opcode_len as usize];
        let get_param = |i| params.get(i).cloned().map(|x| x as usize);

        Some(match opcode_code {
            opcode::OPCODE_ADD_CODE => OPCode::Add {
                addr_add_1: get_param(0)?,
                addr_add_2: get_param(1)?,
                addr_result: get_param(2)?,
            },
            opcode::OPCODE_SUB_CODE => OPCode::Sub {
                addr_sub_1: get_param(0)?,
                addr_sub_2: get_param(1)?,
                addr_result: get_param(2)?,
            },
            opcode::OPCODE_STOP_CODE => OPCode::Stop,
            _ => return None,
        })
    }

    pub fn set(&mut self, index: usize, value: i64) {
        if index >= self.len() {
            panic!("Set value out of bounds: {}", index);
        }
        self.raw_instructions[index] = value;
    }

    pub fn read(&self, index: usize) -> Option<i64> {
        self.raw_instructions.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.raw_instructions.len()
    }
}
