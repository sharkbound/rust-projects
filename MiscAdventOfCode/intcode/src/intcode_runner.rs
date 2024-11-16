use crate::io::IO;
use crate::{Instructions, Memory, Mode, OPCode, State};

pub struct IntCodeRunner {
    pub memory: Memory,
    pub pc: usize,
    pub instructions: Instructions,
    pub id: u32,
    pub state: State,
    pub mode: Mode,
}

impl IntCodeRunner {
    pub fn new(memory: Memory, instructions: Instructions, id: u32) -> Self {
        IntCodeRunner {
            memory,
            pc: 0,
            instructions,
            id,
            state: State::NotStarted,
            mode: Mode::Position,
        }
    }

    pub fn offset_ptr(&self, index: usize) -> usize {
        if index < self.instructions.len() {
            return index;
        }
        index - self.instructions.len()
    }

    pub fn write(&mut self, index: usize, value: i64) {
        if index < self.instructions.len() {
            self.instructions.set(index, value);
        } else {
            self.memory.set(self.offset_ptr(index), value);
        }
    }

    pub fn read(&self, index: usize) -> Option<i64> {
        if index < self.instructions.len() {
            self.instructions.read(index)
        } else {
            self.memory.get_direct(index - ())
        }
    }

    pub fn step_once(&mut self, io: &mut dyn IO) -> State {
        let opcode = match self.instructions.read_opcode(self.pc) {
            Some(opcode) => opcode,
            None => {
                panic!("Could not read opcode on IntCodeRunner#{}", self.id);
            }
        };

        match opcode {
            OPCode::Add {
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

            OPCode::Sub {
                addr_sub_1,
                addr_sub_2,
                addr_result,
            } => {
                let val_sub_1 = self.memory.get_direct(addr_sub_1).expect(&format!(
                    "Could not read value for SUB on IntCodeRunner#{} for arg 1",
                    self.id
                ));
                let val_sub_2 = self.memory.get_direct(addr_sub_2).expect(&format!(
                    "Could not read value for SUB on IntCodeRunner#{} for arg 2",
                    self.id
                ));
                self.memory.set(addr_result, val_sub_1 - val_sub_2);
                self.pc += opcode.opcode_len() as usize + 1;
            }

            OPCode::Stop => {
                self.state = State::Halted;
            }
        }

        self.state
    }

    pub fn step_n_times(&mut self, io: &mut dyn IO, n: usize) -> State {
        for _ in 0..n {
            self.step_once(io);
        }
        self.state
    }

    pub fn run_until_halted(&mut self, io: &mut dyn IO) {
        while self.step_once(io) != State::Halted {}
    }

    pub fn run_until_state(&mut self, io: &mut dyn IO, stop_state: State) {
        while self.step_once(io) != stop_state {}
    }
}
