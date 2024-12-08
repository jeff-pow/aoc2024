use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const GUARD_UP: char = '^';
const GUARD_LEFT: char = '<';
const GUARD_RIGHT: char = 'v';
const GUARD_DOWN: char = '>';
const GUARDS: [char; 4] = [GUARD_UP, GUARD_LEFT, GUARD_DOWN, GUARD_RIGHT];
const OBSTRUCTION: char = '#';
const EMPTY: char = '.';
const VISITED: char = 'X';

fn main() {
    let mut grid = Vec::new();
    let fp = File::open("./input.txt").unwrap();
    //let fp = File::open("./small_input.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }
    dbg!(part2(grid));
}

fn is_loop(mut grid: Vec<Vec<char>>) -> bool {
    let mut guard_x = usize::MAX;
    let mut guard_y = usize::MAX;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if GUARDS.contains(&grid[x][y]) {
                guard_x = x;
                guard_y = y;
                break;
            }
        }
    }

    let og_guard_x = guard_x;
    let og_guard_y = guard_y;
    let og_guard_state = grid[guard_x][guard_y];

    let mut visited = HashSet::new();
    while let Some((new_x, new_y)) = movement_delta(&grid, guard_x, guard_y) {
        let current_state = (guard_x, guard_y, grid[guard_x][guard_y]);

        if visited.contains(&current_state) {
            return true;
        }

        visited.insert(current_state);

        match grid[new_x][new_y] {
            EMPTY => {
                grid[new_x][new_y] = grid[guard_x][guard_y];
                grid[guard_x][guard_y] = VISITED;
                guard_x = new_x;
                guard_y = new_y;
            }
            VISITED => {
                grid[new_x][new_y] = grid[guard_x][guard_y];
                grid[guard_x][guard_y] = VISITED;
                guard_x = new_x;
                guard_y = new_y;
            }
            OBSTRUCTION => grid[guard_x][guard_y] = rotate_guard(grid[guard_x][guard_y]),
            _ => unreachable!(),
        };
    }
    false
}

fn part2(grid: Vec<Vec<char>>) -> i32 {
    let mut guard_x = usize::MAX;
    let mut guard_y = usize::MAX;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if GUARDS.contains(&grid[x][y]) {
                guard_x = x;
                guard_y = y;
                break;
            }
        }
    }
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut grid = grid.clone();
            if grid[x][y] != EMPTY {
                continue;
            }
            grid[x][y] = OBSTRUCTION;
            if is_loop(grid) {
                count += 1;
            }
        }
    }
    count
}

fn part1(mut grid: Vec<Vec<char>>) -> i32 {
    let mut guard_x = usize::MAX;
    let mut guard_y = usize::MAX;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if GUARDS.contains(&grid[x][y]) {
                guard_x = x;
                guard_y = y;
                break;
            }
        }
    }
    let mut positions_visited = 0;
    while let Some((new_x, new_y)) = movement_delta(&grid, guard_x, guard_y) {
        match grid[new_x][new_y] {
            EMPTY => {
                grid[new_x][new_y] = grid[guard_x][guard_y];
                grid[guard_x][guard_y] = VISITED;
                guard_x = new_x;
                guard_y = new_y;
                positions_visited += 1;
            }
            VISITED => {
                grid[new_x][new_y] = grid[guard_x][guard_y];
                grid[guard_x][guard_y] = VISITED;
                guard_x = new_x;
                guard_y = new_y;
            }
            OBSTRUCTION => grid[guard_x][guard_y] = rotate_guard(grid[guard_x][guard_y]),
            _ => unreachable!(),
        }
    }
    positions_visited + 1
}

fn rotate_guard(guard: char) -> char {
    match guard {
        GUARD_DOWN => GUARD_LEFT,
        GUARD_UP => GUARD_RIGHT,
        GUARD_LEFT => GUARD_UP,
        GUARD_RIGHT => GUARD_DOWN,
        _ => unreachable!(),
    }
}

fn movement_delta(grid: &[Vec<char>], guard_x: usize, guard_y: usize) -> Option<(usize, usize)> {
    let guard = grid[guard_x][guard_y];
    match guard {
        GUARD_UP => {
            if guard_x > 0 {
                Some((guard_x - 1, guard_y))
            } else {
                None
            }
        }
        GUARD_LEFT => {
            if guard_y > 0 {
                Some((guard_x, guard_y - 1))
            } else {
                None
            }
        }
        GUARD_RIGHT => {
            if guard_y < grid[0].len() - 1 {
                Some((guard_x, guard_y + 1))
            } else {
                None
            }
        }
        GUARD_DOWN => {
            if guard_x < grid.len() - 1 {
                Some((guard_x + 1, guard_y))
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}
