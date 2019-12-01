use std::fs::File;
use std::io::Read;

fn get_fuel(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + get_fuel(fuel)
}

fn main() {
    let mut f = File::open("data.txt").expect("could not read data.txt");
    let mut text = String::new();

    f.read_to_string(&mut text).expect("could not read contents of data.txt");

    let total: i64 = text.lines().map(|s| get_fuel(s.parse::<i64>().unwrap())).sum();
    println!("{:?}", total);
}