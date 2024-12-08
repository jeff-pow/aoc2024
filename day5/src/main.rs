use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Rules is a map s.t. rules.get(x) returns all the pages that must be *before* x for the order to be valid
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let fp = File::open("./input.txt").unwrap();
    //let fp = File::open("./small_input.txt").unwrap();
    let mut parsing_rules = true;
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let (x, y) = line.split_once('|').unwrap();
            rules
                .entry(y.parse::<i32>().unwrap())
                .or_insert(vec![])
                .push(x.parse::<i32>().unwrap());
        } else {
            updates.push(
                line.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        }
    }
    dbg!(&updates);
    dbg!(part2(rules, updates));
}

fn valid_update(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut seen = HashSet::new();
    for &x in update {
        if let Some(before) = rules.get(&x) {
            for b in before {
                if update.contains(b) && !seen.contains(b) {
                    return false;
                }
            }
        }
        seen.insert(x);
    }
    true
}

fn sort(a: &i32, b: &i32, rules: &HashMap<i32, Vec<i32>>) -> Ordering {
    if let Some(vec) = rules.get(b) {
        if vec.contains(a) {
            return Ordering::Less;
        }
    }
    if let Some(vec) = rules.get(a) {
        if vec.contains(b) {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn part2(rules: HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|&update| !valid_update(update, &rules))
        .map(|update| {
            let mut u = update.clone();
            u.sort_by(|a, b| sort(a, b, &rules));
            assert!(valid_update(&u, &rules));
            u
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
}

fn part1(rules: HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|update| valid_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
}
