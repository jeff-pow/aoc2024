use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut reports = vec![];
    let fp = File::open("./src/report.txt").unwrap();
    //let fp = File::open("./src/small_report.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        reports.push(report);
    }
    dbg!(part2(reports));
}

fn part2(reports: Vec<Vec<i32>>) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            (0..report.len()).any(|idx| {
                is_valid_report(
                    report[..idx]
                        .iter()
                        .cloned()
                        .chain(report[idx + 1..].iter().cloned())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            })
        })
        .count()
}

fn is_valid_report(report: &[i32]) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|(&x, &y)| (1..=3).contains(&(x - y)))
        || report
            .iter()
            .tuple_windows()
            .all(|(&x, &y)| (1..=3).contains(&(y - x)))
}

fn part1(reports: Vec<Vec<i32>>) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            report
                .iter()
                .tuple_windows()
                .all(|(&x, &y)| (1..=3).contains(&(x - y)))
                || report
                    .iter()
                    .tuple_windows()
                    .all(|(&x, &y)| (1..=3).contains(&(y - x)))
        })
        .count()
}
