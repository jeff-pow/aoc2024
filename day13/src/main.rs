use nalgebra::{Cholesky, Matrix2, Vector2};
use std::{cmp::min, fs::read_to_string, io::BufReader};

#[derive(Clone, Copy, Debug)]
struct Game {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn main() {
    let str = read_to_string("example.txt").unwrap();
    let str = read_to_string("input.txt").unwrap();

    let games = parse_games(&str);
    println!("{}", part2(games));
}

const P2_OFFSET: usize = 10_000_000_000_000;

fn part2(mut games: Vec<Game>) -> usize {
    games.iter_mut().for_each(|g| {
        g.prize.0 += P2_OFFSET;
        g.prize.1 += P2_OFFSET;
    });
    games
        .into_iter()
        .filter_map(|game| {
            let a = Matrix2::new(
                game.button_a.0 as f64,
                game.button_b.0 as f64,
                game.button_a.1 as f64,
                game.button_b.1 as f64,
            );
            let b = Vector2::new(game.prize.0 as f64, game.prize.1 as f64);

            if let Some(sol) = a.lu().solve(&b) {
                let a_int = sol[0].round();
                let b_int = sol[1].round();

                let x = a_int * game.button_a.0 as f64 + b_int * game.button_b.0 as f64;
                let y = a_int * game.button_a.1 as f64 + b_int * game.button_b.1 as f64;

                assert!(
                    (x - game.prize.0 as f64).abs() < 1e-6
                        && (y - game.prize.1 as f64).abs() < 1e-6
                );
                if (x - game.prize.0 as f64).abs() < 1e-6 && (y - game.prize.1 as f64).abs() < 1e-6
                {
                    Some((a_int * 3.0 + b_int) as usize)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part1(games: Vec<Game>) -> usize {
    games
        .into_iter()
        .filter_map(|game| {
            let mut min_cost: Option<usize> = None;
            let mut num_solutions = 0;
            for a in 0..100 {
                for b in 0..100 {
                    let x = a * game.button_a.0 + b * game.button_b.0;
                    let y = a * game.button_a.1 + b * game.button_b.1;

                    if x == game.prize.0 && y == game.prize.1 {
                        num_solutions += 1;
                        min_cost = Some(min_cost.map_or(a * 3 + b, |x| x.min(a * 3 + b)));
                    }
                }
            }
            assert!(num_solutions < 2);
            min_cost
        })
        .sum::<usize>()
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    for chunk in lines.chunks(3) {
        let game = Game {
            button_a: parse_coordinate(chunk[0], "Button A: X+", ", Y+"),
            button_b: parse_coordinate(chunk[1], "Button B: X+", ", Y+"),
            prize: parse_coordinate(chunk[2], "Prize: X=", ", Y="),
        };
        games.push(game);
    }

    games
}

fn parse_coordinate(line: &str, prefix: &str, separator: &str) -> (usize, usize) {
    let coords_str = line.trim_start_matches(prefix);

    let parts: Vec<&str> = coords_str.split(separator).collect();

    // Parse the two parts into usize
    let x = parts[0].parse().unwrap();
    let y = parts[1].parse().unwrap();

    (x, y)
}
