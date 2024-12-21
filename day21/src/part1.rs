use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

#[rustfmt::skip]
const NUMERIC_KEYPAD: [[Option<char>; 3]; 4] = [
    [Some('7'), Some('8'), Some('9')],
    [Some('4'), Some('5'), Some('6')],
    [Some('1'), Some('2'), Some('3')],
    [None,      Some('0'), Some('A')],
];

#[rustfmt::skip]
const DIRECTIONAL_KEYPAD: [[Option<char>; 3]; 2] = [
    [None,      Some('^'), Some('A')],
    [Some('<'), Some('v'), Some('>')],
];

fn grid_to_hashmap<const ROWS: usize, const COLS: usize>(
    grid: [[Option<char>; COLS]; ROWS],
) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();

    for row in 0..ROWS {
        for col in 0..COLS {
            if let Some(value) = grid[row][col] {
                map.insert((row as i32, col as i32), value);
            }
        }
    }

    map
}

fn find_path_numeric_bfs(
    locations: &HashMap<(i32, i32), char>,
    start_pos: (i32, i32),
    to_find: char,
) -> Option<String> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), String)> = VecDeque::new();

    queue.push_back((start_pos, "".to_string()));
    visited.insert(start_pos);

    while let Some((pos, path)) = queue.pop_front() {
        if locations.get(&pos).is_some_and(|&c| c == to_find) {
            return Some(path);
        }

        for neighbor in [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
        {
            if locations.contains_key(&neighbor) && !visited.contains(&neighbor) {
                visited.insert(neighbor);
                let dir_travelled = if neighbor.0 > pos.0 {
                    'v'
                } else if neighbor.0 < pos.0 {
                    '^'
                } else if neighbor.1 > pos.1 {
                    '>'
                } else if neighbor.1 < pos.1 {
                    '<'
                } else {
                    unreachable!()
                };
                queue.push_back((neighbor, path.clone() + &dir_travelled.to_string()));
            }
        }
    }

    None
}

fn sort_path(path: &str, sort_type: SortType) -> String {
    let vec = path.chars().collect::<Vec<_>>();
    assert_ne!(vec.last(), Some('A').as_ref());
    let leri = path
        .chars()
        .filter(|&c| matches!(c, '<' | '>'))
        .collect::<Vec<_>>();
    let updo = path
        .chars()
        .filter(|&c| matches!(c, '^' | 'v'))
        .collect::<Vec<_>>();
    assert!(updo.iter().collect::<HashSet<_>>().len() <= 1);
    assert!(leri.iter().collect::<HashSet<_>>().len() <= 1);
    match sort_type {
        SortType::Leri => return leri.iter().chain(updo.iter()).collect(),
        SortType::Updo => return updo.iter().chain(leri.iter()).collect(),
        SortType::Normal => (),
    }
    if leri.is_empty() {
        return updo.iter().collect();
    } else if updo.is_empty() {
        return leri.iter().collect();
    }
    if path.contains('^') && path.contains('<') {
        leri.iter().chain(updo.iter()).collect()
    } else if path.contains('v') && path.contains('<') {
        leri.iter().chain(updo.iter()).collect()
    } else if path.contains('v') && path.contains('>') {
        updo.iter().chain(leri.iter()).collect()
    } else if path.contains('^') && path.contains('>') {
        updo.iter().chain(leri.iter()).collect()
    } else {
        unreachable!("{}", path)
    }
}

#[derive(Clone, Copy, Debug)]
enum SortType {
    Leri,
    Updo,
    Normal,
}

pub fn part1(str: &str) -> usize {
    let numeric_locations = grid_to_hashmap(NUMERIC_KEYPAD);
    let mut numeric_paths = HashMap::new();
    for &from in NUMERIC_KEYPAD.as_flattened().iter().flatten() {
        for &to in NUMERIC_KEYPAD.as_flattened().iter().flatten() {
            if from == to {
                numeric_paths.insert((from, to), "A".to_string());
                continue;
            }
            let mut from_pos = (i32::MAX, i32::MAX);
            let mut to_pos = (i32::MAX, i32::MAX);
            for (&k, &v) in &numeric_locations {
                if from == v {
                    from_pos = k;
                } else if to == v {
                    to_pos = k;
                }
            }
            let raw_path = find_path_numeric_bfs(&numeric_locations, from_pos, to).unwrap();
            let sort_type = if ['0', 'A'].contains(&from) && ['7', '4', '1'].contains(&to) {
                SortType::Updo
            } else if ['7', '4', '1'].contains(&from) && ['0', 'A'].contains(&to) {
                SortType::Leri
            } else {
                SortType::Normal
            };
            let bfs_path = sort_path(&raw_path, sort_type);
            numeric_paths.insert((from, to), bfs_path + "A");
        }
    }

    let directional_locations = grid_to_hashmap(DIRECTIONAL_KEYPAD);
    let mut directional_paths = HashMap::new();
    for &from in DIRECTIONAL_KEYPAD.as_flattened().iter().flatten() {
        for &to in DIRECTIONAL_KEYPAD.as_flattened().iter().flatten() {
            if from == to {
                directional_paths.insert((from, to), "A".to_string());
                continue;
            }
            let mut from_pos = (i32::MAX, i32::MAX);
            let mut to_pos = (i32::MAX, i32::MAX);
            for (&k, &v) in &directional_locations {
                if from == v {
                    from_pos = k;
                } else if to == v {
                    to_pos = k;
                }
            }
            let raw_path = find_path_numeric_bfs(&directional_locations, from_pos, to).unwrap();
            let sort_type = if from == '<' {
                SortType::Leri
            } else if to == '<' {
                SortType::Updo
            } else {
                SortType::Normal
            };
            let bfs_path = sort_path(&raw_path, sort_type);
            directional_paths.insert((from, to), bfs_path + "A");
        }
    }

    str.lines()
        .map(|code| {
            let mut curr = 'A';
            let mut acc = "".to_string();
            for val in code.chars() {
                acc += numeric_paths.get(&(curr, val)).unwrap();
                curr = val;
            }
            dbg!(&acc);

            for _ in 0..2 {
                let mut new_acc = "".to_string();
                let mut curr = 'A';
                for val in acc.chars() {
                    new_acc += directional_paths.get(&(curr, val)).unwrap();
                    curr = val;
                }
                acc = new_acc;
            }
            dbg!(&acc);
            dbg!(acc.len());
            println!();

            code.strip_suffix('A').unwrap().parse::<usize>().unwrap() * acc.len()
        })
        .sum()
}
