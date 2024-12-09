use core::num;
use std::{fs::read_to_string, ops::Range, thread::sleep, time::Duration};

fn main() {
    let input = read_to_string("example.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();
    let str = parse_input(input);

    println!("{}", part2(str));
}

fn find_free_chunk(fs: &[String], required_size: usize) -> Option<usize> {
    fs.windows(required_size)
        .position(|w| w.iter().all(|x| *x == EMPTY))
}

fn find_required_chunk(fs: &[String], num_to_find: u64) -> Range<usize> {
    let start = fs
        .iter()
        .position(|s| *s == num_to_find.to_string())
        .unwrap();
    start
        ..start
            + fs[start..]
                .iter()
                .take_while(|s| **s == num_to_find.to_string())
                .count()
}

fn part2(mut fs: Vec<String>) -> u64 {
    let mut num_to_find = fs
        .iter()
        .filter_map(|x| x.parse::<u64>().ok())
        .max()
        .unwrap();
    let mut chunk = find_required_chunk(&fs, num_to_find);
    loop {
        if let Some(free) = find_free_chunk(&fs[..chunk.start], chunk.len()) {
            for offset in 0..chunk.len() {
                fs.swap(free + offset, chunk.start + offset);
            }
        }
        if num_to_find == 0 {
            break;
        }
        num_to_find -= 1;
        chunk = find_required_chunk(&fs, num_to_find);
    }

    fs.into_iter()
        .enumerate()
        .filter_map(|(idx, c)| c.parse::<u64>().ok().map(|x| x * idx as u64))
        .sum::<u64>()
}

fn find_free(fs: &[String], start: usize) -> usize {
    fs.iter()
        .enumerate()
        .skip(start)
        .find(|c| *c.1 == EMPTY)
        .map(|c| c.0)
        .unwrap()
}

fn find_allocated(fs: &[String]) -> usize {
    fs.len() - 1 - fs.iter().rev().position(|c| c != EMPTY).unwrap()
}

#[allow(dead_code)]
fn part1(mut fs: Vec<String>) -> u64 {
    let mut free = find_free(&fs, 0);
    let mut alloc = find_allocated(&fs);
    while free < alloc {
        fs.swap(free, alloc);
        free = find_free(&fs, free + 1);
        alloc = find_allocated(&fs);
    }

    fs.into_iter()
        .enumerate()
        .filter_map(|(idx, c)| c.parse::<u64>().ok().map(|x| x * idx as u64))
        .sum::<u64>()
}

fn parse_input(input: String) -> Vec<String> {
    let mut vec = vec![];
    let mut file = true;
    let mut file_num = 0;

    for num in input.trim().chars() {
        if file {
            vec.extend(
                std::iter::repeat(file_num.to_string()).take(num.to_digit(10).unwrap() as usize),
            );
            file_num += 1;
        } else {
            vec.extend(
                std::iter::repeat(EMPTY.to_string()).take(num.to_digit(10).unwrap() as usize),
            );
        }
        file = !file;
    }

    vec
}

const EMPTY: &str = ".";
