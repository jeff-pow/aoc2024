use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut first = vec![];
    let mut second = vec![];
    let fp = File::open("./input.txt").unwrap();
    for (line_num, line) in BufReader::new(fp).lines().enumerate() {
        let line = line.unwrap();
        let (f, s) = line.split_once("   ").unwrap();
        dbg!(f, s);
        first.push((f.to_string().parse::<usize>().unwrap(), line_num));
        second.push((s.to_string().parse::<usize>().unwrap(), line_num));
    }
    dbg!(part2(first, second));
}

fn part2(first: Vec<(usize, usize)>, second: Vec<(usize, usize)>) -> usize {
    let mut occurrence_map = HashMap::new();
    for (x, _) in second {
        *occurrence_map.entry(x).or_insert(0) += 1;
    }
    first
        .into_iter()
        .map(|(x, _)| x * occurrence_map.get(&x).unwrap_or(&0))
        .sum::<usize>()
}

fn part1(mut first: Vec<(usize, usize)>, mut second: Vec<(usize, usize)>) -> usize {
    first.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    second.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.0.abs_diff(b.0))
        .inspect(|x| println!("{x}"))
        .sum::<usize>()
}
