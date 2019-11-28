use std::fmt;
use std::io::{stdin, Read, stdout, Write};


pub fn ask(prompt: &str, newline: bool) -> String {
    if newline { println!("{}", prompt); } else { print!("{}", prompt); }
    flush();
    let mut input = String::new();
    stdin().read_line(&mut input).ok();
    input
}

pub fn flush() {
    stdout().flush().ok();
}

pub fn read(prompt: &str, newline: bool) -> char {
    if newline { println!("{}", prompt); } else { print!("{}", prompt); }
    flush();
    let mut i: [u8; 1] = [0];
    stdin().read(&mut i).ok();
    char::from(i[0])
}
