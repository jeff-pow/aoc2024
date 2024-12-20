use pathfinding::prelude::{astar, astar_bag};
use std::{collections::HashSet, fs::File, io::read_to_string};

fn main() {
    let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    //let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let mut grid = Vec::new();

    for line in str.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    println!("{}", part1(&grid));
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
        time: 0,
        time_to_cheat: usize::MAX,
    };
    let mut cheats_used = HashSet::new();

    let successors = |s: &State| neighbors_with_cost(grid, &cheats_used, *s);
    let heuristic = |s: &State| ((s.pos.0.abs_diff(exit_x)) + (s.pos.1.abs_diff(exit_y))) as u32;
    let success = |s: &State| s.pos == (exit_x, exit_y);
    let (path, base_cost) = astar(&start, successors, heuristic, success).unwrap();
    draw_path(grid, &path);
    dbg!(base_cost);

    let vec = (0..base_cost as usize)
        .filter_map(|time_to_cheat| {
            let mut current_path = HashSet::new();
            let successors = |s: &State| neighbors_with_cost(grid, &cheats_used, *s);
            let start = State {
                pos: (start_x, start_y),
                time: 0,
                time_to_cheat,
            };
            let (path, cost) = astar(&start, successors, heuristic, success).unwrap();
            if time_to_cheat == 20 {
                println!("Saved {}", base_cost - cost);
                draw_path(grid, &path);
            }
            if cost < base_cost {
                for window in path.windows(3) {
                    if grid[window[1].pos.1][window[1].pos.0] == '#' {
                        cheats_used.insert([window[0].pos, window[2].pos]);
                    }
                }
                Some(base_cost - cost)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for i in 0..100 {
        if vec.contains(&i) {
            println!(
                "There are {} cheats that save {} picoseconds.",
                vec.iter().filter(|&&x| x == i).count(),
                i
            );
        }
    }
    0
}

fn neighbors_with_cost(
    grid: &[Vec<char>],
    searched: &HashSet<[(usize, usize); 2]>,
    State {
        pos,
        time,
        time_to_cheat,
    }: State,
) -> Vec<(State, u32)> {
    let possible_neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
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
                    time: time + 1,
                    time_to_cheat,
                },
                1,
            ))
        })
        .collect::<Vec<_>>();

    if time == time_to_cheat {
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

fn draw_path(grid: &[Vec<char>], path: &[State]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if path.iter().any(|s| s.pos == (x, y)) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    time: usize,
    time_to_cheat: usize,
}
