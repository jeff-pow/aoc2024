use std::{collections::HashMap, fs::File, io::read_to_string, time::Instant};

fn main() {
    let t = Instant::now();
    //let input = read_to_string(File::open("example.txt").unwrap()).unwrap();
    let input = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(",").map(|p| p.trim()).collect::<Vec<_>>();
    let designs = designs.lines().map(|d| d.trim()).collect::<Vec<_>>();

    println!("{}", part1(&designs, &patterns));
    println!("{}", part2(&designs, &patterns));
    dbg!(t.elapsed());
}

fn part2(designs: &[&str], patterns: &[&str]) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|&p| valid_pattern_p2(p.to_string(), patterns, &mut cache))
        .sum()
}

fn valid_pattern_p2(
    remaining: String,
    patterns: &[&str],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&ret) = cache.get(&remaining) {
        return ret;
    }
    if remaining.is_empty() {
        return 1;
    }
    let ret = patterns
        .iter()
        .map(|pattern| {
            if remaining.ends_with(pattern) {
                valid_pattern_p2(
                    remaining[..remaining.len() - pattern.len()].to_string(),
                    patterns,
                    cache,
                )
            } else {
                0
            }
        })
        .sum();
    cache.insert(remaining.clone(), ret);
    ret
}

fn part1(designs: &[&str], patterns: &[&str]) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|&&p| valid_pattern(p.to_string(), patterns, &mut cache))
        .count()
}

fn valid_pattern(remaining: String, patterns: &[&str], cache: &mut HashMap<String, bool>) -> bool {
    if let Some(ret) = cache.get(&remaining) {
        return *ret;
    }
    if remaining.is_empty() {
        return true;
    }
    let ret = patterns.iter().any(|pattern| {
        remaining.ends_with(pattern)
            && valid_pattern(
                remaining[..remaining.len() - pattern.len()].to_string(),
                patterns,
                cache,
            )
    });
    cache.insert(remaining.clone(), ret);
    ret
}
