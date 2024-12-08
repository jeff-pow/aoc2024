use regex::Regex;
use std::{fs::File, io::Read};

fn main() {
    let mut fp = File::open("./input.txt").unwrap();
    let mut str = String::new();
    let _ = fp.read_to_string(&mut str);
    dbg!(&str);
    dbg!(part2(&str));
}

fn part2(str: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_dont_regex = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut mul_enabled = true; // Multiplication is enabled by default

    let do_dont_matches = do_dont_regex
        .find_iter(str)
        .map(|m| (m.start(), m.as_str()))
        .collect::<Vec<_>>();

    let mut last_processed_index = 0;
    for (index, instruction) in do_dont_matches.iter() {
        let segment = &str[last_processed_index..*index];
        if mul_enabled {
            total += mul_regex
                .captures_iter(segment)
                .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
                .sum::<usize>();
        }

        mul_enabled = *instruction == "do()";

        last_processed_index = *index + instruction.len();
    }
    let remaining_segment = &str[last_processed_index..];
    if mul_enabled {
        total += mul_regex
            .captures_iter(remaining_segment)
            .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
            .sum::<usize>();
    }

    total
}

fn part1(str: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_regex
        .captures_iter(str)
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .sum()
}
