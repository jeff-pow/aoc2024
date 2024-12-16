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

    let (came_from, cost_so_far, goal) = astar_p2(grid, start, (exit_x, exit_y));
    println!("{}", cost_so_far.get(&goal).unwrap());

    let paths = astar_multiple_paths(grid, start, (exit_x, exit_y));

    paths
    //paths
    //    .into_iter()
    //    .flat_map(|(came_from, _, goal)| reconstruct_path(came_from, start, goal))
    //    .map(|s| s.pos)
    //    .collect::<HashSet<_>>()
    //    .len()
}

fn reconstruct_paths(
    came_from: &HashMap<State, Vec<State>>,
    start: State,
    goal: State,
) -> Vec<Vec<State>> {
    let mut paths = vec![];
    let mut q = vec![(goal, vec![goal])];

    while let Some((current, path)) = q.pop() {
        if current == start {
            let mut reversed_path = path.clone();
            reversed_path.reverse();
            paths.push(reversed_path);
            continue;
        }

        if let Some(parents) = came_from.get(&current) {
            for &parent in parents {
                let mut new_path = path.clone();
                new_path.push(parent);
                q.push((parent, new_path));
            }
        }
    }

    paths
}
fn astar_multiple_paths(grid: &[Vec<char>], start: State, goal: (usize, usize)) -> usize {
    let mut all_optimal_paths = HashSet::new();
    let mut frontier = PriorityQueue::new();
    frontier.push(start, Reverse(0));

    let mut cost_so_far: HashMap<State, Vec<usize>> = HashMap::new();
    cost_so_far.insert(start, vec![0]);

    let mut came_from: HashMap<State, Vec<State>> = HashMap::new();
    came_from.insert(start, vec![]);

    let mut min_goal_cost = usize::MAX;

    while let Some((current, _)) = frontier.pop() {
        let current_costs = cost_so_far.get(&current).unwrap().clone();
        let current_cost = current_costs.iter().min().unwrap();

        if current.pos == goal {
            println!("{}", current_cost);
            if *current_cost <= min_goal_cost {
                let paths = reconstruct_paths(&came_from, start, current);

                if *current_cost < min_goal_cost {
                    all_optimal_paths.clear();
                    min_goal_cost = *current_cost;
                }
                for path in paths {
                    draw_path(grid, &path);
                    println!();
                    all_optimal_paths.extend(path.into_iter().map(|s| s.pos));
                }
            }
            continue; // Continue to find other optimal paths
        }

        for next in neighbors(grid, current) {
            let new_cost = current_cost + if next.pos != current.pos { 1 } else { 1000 };

            let next_costs = cost_so_far.entry(next).or_insert(vec![]);
            let min_next_cost = next_costs.iter().min().cloned().unwrap_or(usize::MAX);

            if new_cost <= min_next_cost {
                if new_cost < min_next_cost {
                    next_costs.clear();
                    came_from.entry(next).or_insert(vec![]).clear();
                }
                next_costs.push(new_cost);

                let priority = new_cost + heuristic(next.pos, goal);
                frontier.push(next, Reverse(priority));

                came_from.entry(next).or_insert(vec![]).push(current);
            }
        }
    }

    all_optimal_paths.len()
}
//fn astar_multiple_paths(grid: &[Vec<char>], start: State, goal: (usize, usize)) -> usize {
//    let mut all_optimal_paths = HashSet::new();
//    let mut frontier = PriorityQueue::new();
//    frontier.push(start, Reverse(0));
//    let mut cost_so_far = HashMap::new();
//    cost_so_far.insert(start, 0);
//    let mut came_from = HashMap::new();
//    came_from.insert(start, None);
//    let mut min_goal_cost = usize::MAX;
//
//    while let Some((current, _)) = frontier.pop() {
//        if current.pos == goal {
//            let current_cost = *cost_so_far.get(&current).unwrap();
//
//            let path = reconstruct_path(came_from.clone(), start, current);
//            println!("{}", current_cost);
//            draw_path(grid, &path);
//            println!();
//            if current_cost <= min_goal_cost {
//                if current_cost < min_goal_cost {
//                    all_optimal_paths.clear();
//                    min_goal_cost = current_cost;
//                }
//                //draw_path(grid, &path);
//                all_optimal_paths.extend(path.into_iter().map(|s| s.pos));
//                //all_optimal_paths.push((came_from.clone(), cost_so_far.clone(), current));
//            } else {
//                //break;
//            }
//        }
//
//        for next in neighbors(grid, current) {
//            let new_cost =
//                cost_so_far.get(&current).unwrap() + if next.pos != current.pos { 1 } else { 1000 };
//
//            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
//                cost_so_far.insert(next, new_cost);
//
//                let priority = new_cost + heuristic(next.pos, goal);
//                frontier.push(next, Reverse(priority));
//
//                came_from.insert(next, Some(current));
//            }
//        }
//    }
//
//    all_optimal_paths.len()
//}

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

fn astar_p2(
    grid: &[Vec<char>],
    start: State,
    goal: (usize, usize),
) -> (HashMap<State, Option<State>>, HashMap<State, usize>, State) {
    let mut frontier = PriorityQueue::new();
    frontier.push(start, Reverse(0));
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);
    let mut came_from = HashMap::new();
    came_from.insert(start, None);

    while let Some((current, _)) = frontier.pop() {
        if current.pos == goal {
            return (came_from, cost_so_far, current);
        }
        for next in neighbors(grid, current) {
            let new_cost =
                cost_so_far.get(&current).unwrap() + if next.pos != current.pos { 1 } else { 1000 };

            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);

                let priority = new_cost + heuristic(next.pos, goal);
                frontier.push(next, Reverse(priority));

                came_from.insert(next, Some(current));
            }
        }
    }

    unreachable!()
}

fn reconstruct_path(
    came_from: HashMap<State, Option<State>>,
    start: State,
    goal: State,
) -> Vec<State> {
    let mut current = goal;
    let mut path = Vec::new();

    while current != start {
        path.push(current);
        current = came_from.get(&current).unwrap().unwrap();
    }
    path.push(start);
    path.reverse();
    path
}

fn score_path(path: &[State]) -> usize {
    path.windows(2)
        .map(|window| {
            if window[1].pos != window[0].pos {
                1
            } else {
                1000
            }
        })
        .sum()
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
