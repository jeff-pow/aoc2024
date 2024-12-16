use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::read_to_string,
};

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

    astar(
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

fn astar(grid: &[Vec<char>], start: State, goal: (usize, usize)) -> usize {
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

fn find_path(
    grid: &[Vec<char>],
    pos: (usize, usize),
    facing: (isize, isize),
    visited: HashSet<((usize, usize), (isize, isize))>,
    depth_remaining: usize,
) -> Option<u64> {
    if grid[pos.1][pos.0] == 'E' {
        return Some(0);
    }
    if depth_remaining == 0 {
        return None;
    }
    if visited.contains(&(pos, facing)) {
        return None;
    }
    let mut visited = visited.clone();
    visited.insert((pos, facing));

    let new_pos = (
        pos.0.checked_add_signed(facing.0).unwrap(),
        pos.1.checked_add_signed(facing.1).unwrap(),
    );
    if grid[new_pos.1][new_pos.0] != '#' {
        if let Some(score) = find_path(grid, new_pos, facing, visited.clone(), depth_remaining - 1)
        {
            return Some(score + 1);
        }
    }
    let left = rotate_left(facing);
    let left_score = find_path(grid, pos, left, visited.clone(), depth_remaining - 1);

    let right = rotate_left(facing);
    let right_score = find_path(grid, pos, right, visited, depth_remaining - 1);

    match (left_score, right_score) {
        (Some(l), Some(r)) => Some(l.min(r)),
        (Some(l), None) => Some(l + 1000),
        (None, Some(r)) => Some(r + 1000),
        (None, None) => None,
    }
}
