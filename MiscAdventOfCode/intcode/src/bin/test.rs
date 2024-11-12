use intcode::IntCodeRunner::IntCodeRunner;
use intcode::{Instructions, Memory};

fn main() {
    let instructions = Instructions::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    let mut intcode_runner = IntCodeRunner::new(Memory::empty(), &instructions, 0);
    intcode_runner.step_once();
}
