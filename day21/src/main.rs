use part1::part1;
use part2::part2;
use std::{fs::File, io::read_to_string, time::Instant};

mod part1;
mod part2;

// Credit to https://www.reddit.com/r/adventofcode/comments/1hjgyps/2024_day_21_part_2_i_got_greedyish/
// for help with my solution.
fn main() {
    //let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let t = Instant::now();
    println!("{}", part1(&str));
    println!("{}", part2(&str));
    dbg!(t.elapsed());
}
