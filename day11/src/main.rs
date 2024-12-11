use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let x = read_to_string("example.txt").unwrap();
    let x = read_to_string("input.txt").unwrap();
    let vec = x
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let t = Instant::now();
    dbg!(part2(vec));
    dbg!(t.elapsed());
}

fn part2(vec: Vec<u64>) -> usize {
    let mut map = HashMap::new();
    vec.into_iter().for_each(|x| {
        *map.entry(x).or_insert(0) += 1;
    });

    for _ in 0..75 {
        let mut new_map = HashMap::new();
        for (&stone_value, &count) in map.iter() {
            if stone_value == 0 {
                *new_map.entry(1).or_insert(0) += count;
            } else if stone_value.to_string().len() % 2 == 0 {
                let mid = stone_value.to_string().len() / 2;
                *new_map
                    .entry(stone_value.to_string()[..mid].parse().unwrap())
                    .or_insert(0) += count;
                *new_map
                    .entry(stone_value.to_string()[mid..].parse().unwrap())
                    .or_insert(0) += count;
            } else {
                *new_map.entry(stone_value * 2024).or_insert(0) += count;
            }
        }
        map = new_map;
    }

    map.values().sum()
}

fn part1(mut vec: Vec<u64>) -> usize {
    for _ in 0..25 {
        let mut new = Vec::new();
        for x in vec {
            if x == 0 {
                new.push(1);
            } else if x.to_string().len() % 2 == 0 {
                let mid = x.to_string().len() / 2;
                new.push(x.to_string()[..mid].parse().unwrap());
                new.push(x.to_string()[mid..].parse().unwrap());
            } else {
                new.push(x * 2024);
            }
        }
        vec = new;
    }
    vec.len()
}
