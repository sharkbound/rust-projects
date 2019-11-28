use std::io::{Read, stdin, stdout, Write};
use crate::objects::Point;

mod scripts;
mod console;
mod enums;
mod objects;

fn main() {
    let point = Point::new(1, 1);
    let other = Point::new(4, 1);
    println!("{}", point.diff(other));
}


