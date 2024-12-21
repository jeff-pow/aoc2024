use part1::part1;
use part2::part2;
use std::{fs::File, io::read_to_string, time::Instant};

mod part1;
mod part2;

fn main() {
    //let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let t = Instant::now();
    println!("{}", part1(&str));
    println!("{}", part2(&str));
    dbg!(t.elapsed());
}
