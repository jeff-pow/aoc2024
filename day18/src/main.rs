use pathfinding::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::read_to_string,
};

//const INPUT: &str = "input.txt";
//const GRID_DIM: usize = 71;
const INPUT: &str = "example.txt";
const GRID_DIM: usize = 7;

fn main() {
    let str = read_to_string(File::open(INPUT).unwrap()).unwrap();
    let grids = parse_input(&str);

    println!("{}", part1(&grids));
}

fn part1(grids: &[(usize, usize)]) -> usize {
    let start = State {
        pos: (0, 0),
        time: 0,
    };

    let successors = |s: &State| neighbors(grids, *s);
    let heuristic =
        |s: &State| ((s.pos.0.abs_diff(GRID_DIM - 1)) + (s.pos.1.abs_diff(GRID_DIM - 1))) as u32;
    let success = |s: &State| s.pos == (GRID_DIM - 1, GRID_DIM - 1);
    //let (paths, cost) = astar_bag_collect(&start, successors, heuristic, success).unwrap();
    let (path, cost) = astar(&start, successors, heuristic, success).unwrap();
    draw_path(path);
    cost as usize
}

fn draw_path(path: Vec<State>) {
    for y in 0..GRID_DIM {
        for x in 0..GRID_DIM {
            if path.iter().any(|s| s.pos == (x, y)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn neighbors(blockers: &[(usize, usize)], State { pos, time }: State) -> Vec<(State, u32)> {
    let mut possible_neighbors = vec![];
    for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let Some(new_x) = pos.0.checked_add_signed(offset.0) else {
            continue;
        };
        let Some(new_y) = pos.1.checked_add_signed(offset.1) else {
            continue;
        };
        if blockers[0..time].contains(&(new_x, new_y)) {
            continue;
        }
        possible_neighbors.push((
            State {
                pos: (new_x, new_y),
                time: time + 1,
            },
            1,
        ));
    }
    possible_neighbors
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    time: usize,
}

fn parse_input(str: &str) -> Vec<(usize, usize)> {
    let mut ret = vec![(usize::MAX, usize::MAX)];
    for line in str.lines() {
        let (x, y) = line.split_once(',').unwrap();
        ret.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    ret
}
