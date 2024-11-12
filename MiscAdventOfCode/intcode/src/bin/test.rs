use intcode::Instructions;

fn main() {
    let instructions = Instructions::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    println!("{:?}", instructions.read_opcode(0));
}
