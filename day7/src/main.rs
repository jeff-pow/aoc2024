use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let mut equations = Vec::new();
    let fp = File::open("./input.txt").unwrap();
    //let fp = File::open("./small_input.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        let (ans, operators) = line.split_once(':').unwrap();
        equations.push((
            ans.parse::<i64>().unwrap(),
            operators
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        ));
    }

    let t = Instant::now();
    dbg!(equations
        .iter()
        .filter_map(|(ans, operators)| {
            if check_equation(*ans, operators[0], &operators[1..]) {
                Some(ans)
            } else {
                None
            }
        })
        .sum::<i64>());
    dbg!(t.elapsed());

    let t = Instant::now();
    println!("{}", part2(equations));
    dbg!(t.elapsed());
}

fn check_equation(ans: i64, accumulator: i64, remaining: &[i64]) -> bool {
    if remaining.is_empty() && accumulator == ans {
        return true;
    }
    if accumulator > ans || remaining.is_empty() {
        return false;
    }

    check_equation(ans, accumulator + remaining[0], &remaining[1..])
        || check_equation(ans, accumulator * remaining[0], &remaining[1..])
        || check_equation(ans, cat(accumulator, remaining[0]), &remaining[1..])
}

// Thanks ana
fn cat(n: i64, m: i64) -> i64 {
    let mut f = 1;
    while f <= m {
        f *= 10;
    }
    n * f + m
}

// Beware ye who venture beyond here
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn apply(self, accumulator: &mut i64, x: i64) {
        match self {
            Self::Mul => *accumulator *= x,
            Self::Add => *accumulator += x,
            Self::Concat => {
                *accumulator = (accumulator.to_string() + &x.to_string())
                    .parse::<i64>()
                    .unwrap()
            }
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Add => Self::Mul,
            Self::Mul => Self::Concat,
            Self::Concat => Self::Add,
        }
    }
}

fn part1(equations: Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .into_iter()
        .filter(|(ans, operators)| {
            let mut counter = 0u64;
            while counter < 1 << operators.len() {
                let mut accumulator = operators[0];
                for (idx, &x) in operators.iter().enumerate().skip(1) {
                    if counter & (1 << (idx - 1)) != 0 {
                        accumulator *= x
                    } else {
                        accumulator += x
                    }
                }
                if accumulator == *ans {
                    return true;
                }
                counter += 1;
            }
            false
        })
        .map(|(ans, _)| ans)
        .sum::<i64>()
}

struct Counter([Operation; 64]);

impl Counter {
    fn incr(&mut self) {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i].next();
            if self.0[i] != Operation::Add {
                break;
            }
        }
    }
}

fn part2(equations: Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .into_iter()
        .filter(|(ans, operators)| {
            let mut counter = Counter([Operation::Add; 64]);
            while counter.0[operators.len()] == Operation::Add {
                let mut accumulator = operators[0];
                for (idx, &x) in operators.iter().enumerate().skip(1) {
                    counter.0[idx].apply(&mut accumulator, x);
                }
                if accumulator == *ans {
                    return true;
                }
                counter.incr();
            }
            false
        })
        .map(|(ans, _)| ans)
        .sum::<i64>()
}
