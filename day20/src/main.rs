mod part2;

use part2::part2;
use pathfinding::prelude::astar;
use std::{collections::HashSet, fs::File, io::read_to_string};

fn main() {
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let mut grid = Vec::new();

    for line in str.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    println!("{}", part1(&grid));
    println!("{}", part2(&str));
}

fn part1(grid: &[Vec<char>]) -> usize {
    let mut start_x = usize::MAX;
    let mut start_y = usize::MAX;
    let mut exit_x = usize::MAX;
    let mut exit_y = usize::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start_x = x;
                start_y = y;
            } else if grid[y][x] == 'E' {
                exit_x = x;
                exit_y = y;
            }
        }
    }

    let start = State {
        pos: (start_x, start_y),
        has_cheated: true,
    };
    let mut cheats_used = HashSet::new();

    let successors = |s: &State| neighbors_with_cost(grid, &cheats_used, *s);
    let heuristic = |s: &State| ((s.pos.0.abs_diff(exit_x)) + (s.pos.1.abs_diff(exit_y))) as u32;
    let success = |s: &State| s.pos == (exit_x, exit_y);
    let (_, base_cost) = astar(&start, successors, heuristic, success).unwrap();
    dbg!(base_cost);

    std::iter::from_fn(|| {
        let successors = |s: &State| neighbors_with_cost(grid, &cheats_used, *s);
        let start = State {
            pos: (start_x, start_y),
            has_cheated: false,
        };

        astar(&start, successors, heuristic, success)
            .filter(|&(_, cost)| cost < base_cost)
            .map(|(path, cost)| {
                path.windows(3).for_each(|window| {
                    if grid[window[1].pos.1][window[1].pos.0] == '#' {
                        cheats_used.insert([window[0].pos, window[2].pos]);
                    }
                });
                base_cost - cost
            })
    })
    .inspect(|&time_saved| println!("{time_saved}"))
    .take_while(|&time_saved| time_saved >= 100)
    .count()
}

fn neighbors_with_cost(
    grid: &[Vec<char>],
    searched: &HashSet<[(usize, usize); 2]>,
    State { pos, has_cheated }: State,
) -> Vec<(State, u32)> {
    let mut possible_neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|dir| {
            let new_x = pos.0.checked_add_signed(dir.0)?;
            let new_y = pos.1.checked_add_signed(dir.1)?;
            if new_x >= grid[0].len() || new_y >= grid.len() {
                return None;
            }
            if searched
                .iter()
                .any(|[p1, p2]| is_midpoint(*p1, *p2, (new_x, new_y)))
            {
                return None;
            }

            Some((
                State {
                    pos: (new_x, new_y),
                    has_cheated,
                },
                1,
            ))
        })
        .collect::<Vec<_>>();

    if !has_cheated {
        possible_neighbors
            .iter_mut()
            .for_each(|(State { pos, has_cheated }, _)| {
                if grid[pos.1][pos.0] == '#' {
                    *has_cheated = true;
                }
            });
        possible_neighbors
    } else {
        possible_neighbors
            .into_iter()
            .filter(|(State { pos, .. }, _)| grid[pos.1][pos.0] != '#')
            .collect()
    }
}

fn is_midpoint(point1: (usize, usize), point2: (usize, usize), point3: (usize, usize)) -> bool {
    let p1 = (point1.0 as i32, point1.1 as i32);
    let p2 = (point2.0 as i32, point2.1 as i32);
    let p3 = (point3.0 as i32, point3.1 as i32);

    let mid_x = (p1.0 + p2.0) / 2;
    let mid_y = (p1.1 + p2.1) / 2;

    let dist_squared = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
    if dist_squared != 4 {
        return false;
    }

    p3 == (mid_x, mid_y)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    has_cheated: bool,
}
