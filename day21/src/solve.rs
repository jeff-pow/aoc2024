use std::collections::{HashMap, HashSet, VecDeque};

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

    for (i, row) in grid.into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            if let Some(val) = val {
                map.insert((i as i32, j as i32), val);
            }
        }
    }

    map
}

fn find_path(
    locations: &HashMap<(i32, i32), char>,
    start_pos: (i32, i32),
    to_find: char,
) -> Option<String> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), String)> = VecDeque::new();

    queue.push_back((start_pos, String::new()));
    visited.insert(start_pos);

    while let Some((pos, path)) = queue.pop_front() {
        if locations.get(&pos) == Some(&to_find) {
            return Some(path);
        }

        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in neighbors {
            let neighbor_pos = (pos.0 + dr, pos.1 + dc);
            if locations.contains_key(&neighbor_pos) && !visited.contains(&neighbor_pos) {
                visited.insert(neighbor_pos);
                let dir_travelled = match (dr, dc) {
                    (-1, 0) => '^',
                    (1, 0) => 'v',
                    (0, -1) => '<',
                    (0, 1) => '>',
                    _ => unreachable!(),
                };
                queue.push_back((neighbor_pos, path.clone() + &dir_travelled.to_string()));
            }
        }
    }

    None
}

fn sort_path(path: &str, sort_type: SortType) -> String {
    if path.is_empty() {
        return String::new();
    }

    let (leri, updo): (Vec<_>, Vec<_>) = path.chars().partition(|&c| matches!(c, '<' | '>'));
    assert!(updo.iter().collect::<HashSet<_>>().len() <= 1);
    assert!(leri.iter().collect::<HashSet<_>>().len() <= 1);

    match sort_type {
        SortType::Leri => leri.into_iter().chain(updo).collect(),
        SortType::Updo => updo.into_iter().chain(leri).collect(),
        SortType::Normal => {
            match (
                path.contains('^'),
                path.contains('v'),
                path.contains('<'),
                path.contains('>'),
            ) {
                (true, false, true, false) | (false, true, true, false) => {
                    leri.into_iter().chain(updo).collect()
                }
                (false, true, false, true) | (true, false, false, true) => {
                    updo.into_iter().chain(leri).collect()
                }
                (false, false, _, _) => leri.into_iter().collect(),
                (_, _, false, false) => updo.into_iter().collect(),
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SortType {
    Leri,
    Updo,
    Normal,
}

fn len_recursive(
    curr: char,
    next: char,
    cache: &mut HashMap<(char, char, usize), usize>,
    directional_paths: &HashMap<(char, char), String>,
    depth: usize,
) -> usize {
    if let Some(&len) = cache.get(&(curr, next, depth)) {
        return len;
    }
    if depth == 0 {
        return 1;
    }

    let expansion = directional_paths.get(&(curr, next)).unwrap();

    let mut i = 'A';
    let mut total = 0;
    for next in expansion.chars() {
        total += len_recursive(i, next, cache, directional_paths, depth - 1);
        i = next;
    }
    cache.insert((curr, next, depth), total);
    total
}

pub fn part2(str: &str, depth: usize) -> usize {
    let (numeric_paths, directional_paths) = paths();

    let mut cache = HashMap::new();
    str.lines()
        .map(|code| {
            let mut curr = 'A';
            let mut acc = "".to_string();
            for val in code.chars() {
                acc += numeric_paths.get(&(curr, val)).unwrap();
                curr = val;
            }

            let mut total = 0;
            let mut curr = 'A';
            for next in acc.chars() {
                total += len_recursive(curr, next, &mut cache, &directional_paths, depth);
                curr = next;
            }

            code.strip_suffix('A').unwrap().parse::<usize>().unwrap() * total
        })
        .sum()
}

#[allow(clippy::type_complexity)]
fn paths() -> (HashMap<(char, char), String>, HashMap<(char, char), String>) {
    let numeric_locations = grid_to_hashmap(NUMERIC_KEYPAD);
    let mut numeric_paths = HashMap::new();
    for &from in NUMERIC_KEYPAD.as_flattened().iter().flatten() {
        for &to in NUMERIC_KEYPAD.as_flattened().iter().flatten() {
            if from == to {
                numeric_paths.insert((from, to), "A".to_string());
                continue;
            }
            for (&k, &v) in &numeric_locations {
                if from == v {
                    let raw_path = find_path(&numeric_locations, k, to).unwrap();
                    let sort_type = if ['0', 'A'].contains(&from) && ['7', '4', '1'].contains(&to) {
                        SortType::Updo
                    } else if ['7', '4', '1'].contains(&from) && ['0', 'A'].contains(&to) {
                        SortType::Leri
                    } else {
                        SortType::Normal
                    };
                    let bfs_path = sort_path(&raw_path, sort_type);
                    numeric_paths.insert((from, to), bfs_path + "A");
                    break;
                }
            }
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
            for (&k, &v) in &directional_locations {
                if from == v {
                    let raw_path = find_path(&directional_locations, k, to).unwrap();
                    let sort_type = if from == '<' {
                        SortType::Leri
                    } else if to == '<' {
                        SortType::Updo
                    } else {
                        SortType::Normal
                    };
                    let bfs_path = sort_path(&raw_path, sort_type);
                    directional_paths.insert((from, to), bfs_path + "A");
                    break;
                }
            }
        }
    }
    (numeric_paths, directional_paths)
}
