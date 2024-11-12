use crate::{Instructions, Memory, OPCode, State};

pub struct IntCodeRunner<'a> {
    pub memory: Memory,
    pub pc: usize,
    pub instructions: &'a Instructions,
    pub id: u32,
    pub state: State,
}

impl<'a> IntCodeRunner<'a> {
    pub fn new(memory: Memory, instructions: &'a Instructions, id: u32) -> Self {
        IntCodeRunner {
            memory,
            pc: 0,
            instructions,
            id,
            state: State::NotStarted,
        }
    }

    pub fn step_once(&mut self) -> State {
        let opcode = match self.instructions.read_opcode(self.pc) {
            Some(opcode) => opcode,
            None => {
                panic!("Could not read opcode on IntCodeRunner#{}", self.id);
            }
        };

        match opcode {
            OPCode::ADD {
                addr_add_1,
                addr_add_2,
                addr_result,
            } => {
                let val_add_1 = self.memory.get_direct(addr_add_1).expect(&format!(
                    "Could not read value for ADD on IntCodeRunner#{} for arg 1",
                    self.id
                ));
                let val_add_2 = self.memory.get_direct(addr_add_2).expect(&format!(
                    "Could not read value for ADD on IntCodeRunner#{} for arg 2",
                    self.id
                ));
                self.memory.set(addr_result, val_add_1 + val_add_2);
                self.pc += opcode.opcode_len() as usize + 1;
            }

            OPCode::SUB {
                addr_sub_1,
                addr_sub_2,
                addr_result,
            } => {
                let val_sub_1 = self.memory.get_direct(addr_sub_1).expect(&format!(
                    "Could not read value for ADD on IntCodeRunner#{} for arg 1",
                    self.id
                ));
                let val_sub_2 = self.memory.get_direct(addr_sub_2).expect(&format!(
                    "Could not read value for ADD on IntCodeRunner#{} for arg 2",
                    self.id
                ));
                self.memory.set(addr_result, val_sub_1 - val_sub_2);
                self.pc += opcode.opcode_len() as usize + 1;
            }
            OPCode::STOP => {
                self.state = State::Halted;
            }
        }
        self.state
    }
}
