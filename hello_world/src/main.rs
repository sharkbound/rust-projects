use std::io::{Read, stdin, stdout, Write};
use std::fmt::Debug;
use std::collections::LinkedList;

mod scripts;
mod console;
mod enums;
mod traits;
mod utils;
mod point;
mod lambda;

fn main() {
    func(0, 101, |x| x % 2 == 0)
}




