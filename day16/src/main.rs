use pathfinding::prelude::astar_bag_collect;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::read_to_string,
};

fn main() {
    let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let mut grid = Vec::new();

    for line in str.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    println!("{}", part2(&grid));
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut reindeer_x = usize::MAX;
    let mut reindeer_y = usize::MAX;
    let mut exit_x = usize::MAX;
    let mut exit_y = usize::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                reindeer_x = x;
                reindeer_y = y;
            } else if grid[y][x] == 'E' {
                exit_x = x;
                exit_y = y;
            }
        }
    }

    let start = State {
        pos: (reindeer_x, reindeer_y),
        dir: (1, 0),
    };

    let successors = |s: &State| neighbors_with_cost(grid, *s);
    let heuristic = |s: &State| ((s.pos.0.abs_diff(exit_x)) + (s.pos.1.abs_diff(exit_y))) as u32;
    let success = |s: &State| s.pos == (exit_x, exit_y);
    let (paths, _) = astar_bag_collect(&start, successors, heuristic, success).unwrap();

    paths
        .into_iter()
        .flatten()
        .map(|v| v.pos)
        .collect::<HashSet<_>>()
        .len()
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

fn part1(grid: &[Vec<char>]) -> usize {
    let mut reindeer_x = usize::MAX;
    let mut reindeer_y = usize::MAX;
    let mut exit_x = usize::MAX;
    let mut exit_y = usize::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                reindeer_x = x;
                reindeer_y = y;
            } else if grid[y][x] == 'E' {
                exit_x = x;
                exit_y = y;
            }
        }
    }

    astar_p1(
        grid,
        State {
            pos: (reindeer_x, reindeer_y),
            dir: (1, 0),
        },
        (exit_x, exit_y),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    dir: (isize, isize),
}

fn astar_p1(grid: &[Vec<char>], start: State, goal: (usize, usize)) -> usize {
    let mut frontier = PriorityQueue::new();
    frontier.push(start, Reverse(0));
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some((state, _)) = frontier.pop() {
        if state.pos == goal {
            return *cost_so_far.get(&state).unwrap();
        }
        for new_state in neighbors(grid, state) {
            let new_cost = cost_so_far.get(&state).unwrap()
                + if new_state.pos != state.pos { 1 } else { 1000 };

            if !cost_so_far.contains_key(&new_state)
                || new_cost < *cost_so_far.get(&new_state).unwrap()
            {
                cost_so_far.insert(new_state, new_cost);

                let priority = new_cost + heuristic(new_state.pos, goal);

                frontier.push(new_state, Reverse(priority));
            }
        }
    }

    unreachable!()
}

fn neighbors_with_cost(grid: &[Vec<char>], State { pos, dir }: State) -> Vec<(State, u32)> {
    let possible_neighbors = [
        (
            State {
                pos: (
                    pos.0.checked_add_signed(dir.0).unwrap(),
                    pos.1.checked_add_signed(dir.1).unwrap(),
                ),
                dir,
            },
            1,
        ),
        (
            State {
                pos,
                dir: rotate_left(dir),
            },
            1000,
        ),
        (
            State {
                pos,
                dir: rotate_right(dir),
            },
            1000,
        ),
    ];

    possible_neighbors
        .into_iter()
        .filter(|(State { pos, .. }, _)| grid[pos.1][pos.0] != '#')
        .collect()
}

fn neighbors(grid: &[Vec<char>], State { pos, dir }: State) -> Vec<State> {
    let possible_neighbors = [
        State {
            pos: (
                pos.0.checked_add_signed(dir.0).unwrap(),
                pos.1.checked_add_signed(dir.1).unwrap(),
            ),
            dir,
        },
        State {
            pos,
            dir: rotate_left(dir),
        },
        State {
            pos,
            dir: rotate_right(dir),
        },
    ];

    possible_neighbors
        .into_iter()
        .filter(|State { pos, .. }| grid[pos.1][pos.0] != '#')
        .collect()
}

fn heuristic((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
}

fn rotate_right(current_dir: (isize, isize)) -> (isize, isize) {
    match current_dir {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    }
}

fn rotate_left(current_dir: (isize, isize)) -> (isize, isize) {
    match current_dir {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => unreachable!(),
    }
}
