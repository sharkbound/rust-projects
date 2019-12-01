use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("could not read data.txt");
    let mut text = String::new();

    f.read_to_string(&mut text).expect("could not read contents of data.txt");

    let total: i64 = text.lines().map(|s| (s.parse::<i64>().unwrap() / 3) - 2).sum();
    println!("{:?}", total);
}