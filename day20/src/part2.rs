use std::collections::HashMap;

pub fn part2(str: &str) -> i64 {
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut exit = (0, 0);
    for (y, line) in str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i64, y as i64), c);
            if c == 'S' {
                start = (x as i64, y as i64);
            } else if c == 'E' {
                exit = (x as i64, y as i64);
            }
        }
    }

    let mut current = start;
    let mut track = HashMap::new();
    let mut distance_travelled = 0;
    track.insert(start, distance_travelled);
    while current != exit {
        track.insert(current, distance_travelled);
        let next = [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let new_x = current.0 + dx;
                let new_y = current.1 + dy;
                if !track.contains_key(&(new_x, new_y))
                    && grid.get(&(new_x, new_y)).is_some_and(|&val| val != '#')
                {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        current = next;
        distance_travelled += 1;
    }
    track.insert(exit, distance_travelled);

    let mut num_cheats = 0;

    const DIST: i64 = 20;
    for ((x, y), distance_travelled) in &track {
        for (dx, dy) in (-DIST..=DIST).flat_map(move |y: i64| {
            let limit = DIST - y.abs();
            (-limit..=limit).map(move |x| (x, y))
        }) {
            if dy == 0 && dx == 0 {
                continue;
            }
            let (x2, y2) = (x + dx, y + dy);
            if track.contains_key(&(x2, y2)) {
                let saved =
                    track.get(&(x2, y2)).unwrap() - distance_travelled - dx.abs() - dy.abs();
                if saved >= 100 {
                    num_cheats += 1;
                }
            }
        }
    }

    num_cheats
}
