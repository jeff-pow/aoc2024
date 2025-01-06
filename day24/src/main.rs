use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../example.txt");
    let input = include_str!("../larger_example.txt");
    let input = include_str!("../input.txt");
    let (registers, instr) = input.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| {
            let (name, val) = line.split_once(":").unwrap();
            (name, val.trim().parse::<i32>().unwrap())
        })
        .collect::<HashMap<_, _>>();
    println!("{}", part1(registers, instr));
}

fn part2<'a>(mut registers: HashMap<&'a str, i32>, instr: &'a str) -> usize {
    let mut instructions = instr.lines().collect::<VecDeque<_>>();
    let mut z_max = instructions
        .iter()
        .filter(|&&tmp| tmp.split_whitespace().last().unwrap().starts_with('z'))
        .map(|z| z.strip_prefix('z').unwrap().parse::<i32>().unwrap())
        .max()
        .unwrap();
    while let Some(instr) = instructions.pop_front() {
        match instr.split_whitespace().collect::<Vec<_>>()[..] {
            [x, op, y, "->", z] => {
                let Some(&x) = registers.get(x) else {
                    instructions.push_back(instr);
                    continue;
                };
                let Some(&y) = registers.get(y) else {
                    instructions.push_back(instr);
                    continue;
                };
                if z.starts_with('z') {
                    if !(z.strip_prefix('z').unwrap().parse::<i32>().unwrap() == z_max
                        || op == "XOR")
                    {
                        instructions.push_back(instr);
                    }
                }
                registers.insert(
                    z,
                    match op {
                        "AND" => x & y,
                        "OR" => x | y,
                        "XOR" => x ^ y,
                        _ => unreachable!(),
                    },
                );
            }
            _ => unreachable!(),
        }
    }

    todo!()
}

fn part1<'a>(mut registers: HashMap<&'a str, i32>, instr: &'a str) -> usize {
    let mut instructions = instr.lines().collect::<VecDeque<_>>();
    while let Some(instr) = instructions.pop_front() {
        match instr.split_whitespace().collect::<Vec<_>>()[..] {
            [x, op, y, "->", z] => {
                let Some(&x) = registers.get(x) else {
                    instructions.push_back(instr);
                    continue;
                };
                let Some(&y) = registers.get(y) else {
                    instructions.push_back(instr);
                    continue;
                };
                registers.insert(
                    z,
                    match op {
                        "AND" => x & y,
                        "OR" => x | y,
                        "XOR" => x ^ y,
                        _ => unreachable!(),
                    },
                );
            }
            _ => unreachable!(),
        }
    }

    let mut vec = vec![];
    for (k, v) in registers {
        if k.starts_with("z") {
            vec.push((k, v));
        }
    }
    vec.sort_by_key(|x| std::cmp::Reverse(x.0));
    let str = vec.iter().map(|x| x.1.to_string()).collect::<String>();
    usize::from_str_radix(&str, 2).unwrap()
}
