use intcode::intcode_runner::IntCodeRunner;
use intcode::{Instructions, Memory, LIFOIO};
use rustutils::iterable_string_ext::JoinToStringExt;

static INPUT: &str = include_str!("../../inputs/2019-2.txt");
static SAMPLE: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

fn main() {
    let parsed = INPUT
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // println!("Input: {:?}", parsed);
    let mut instr = Instructions::new(parsed.clone());
    instr.set(1, 12);
    instr.set(2, 2);
    let mut intcode = IntCodeRunner::new(Memory::empty(), instr, 0);
    let mut io = LIFOIO::new();
    intcode.run_until_halted(&mut io);
    println!("2019 Day 2: {:?}", intcode.instructions.read(0).unwrap());
    // println!("MEM: {:?}", intcode.instructions.raw_instructions.iter().join_to_string(", ", |x| x.to_string()))
}
