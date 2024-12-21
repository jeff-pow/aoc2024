use part1::part1;
use std::{fs::File, io::read_to_string};

mod part1;

fn main() {
    let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    println!("{}", part1(&str));
}
