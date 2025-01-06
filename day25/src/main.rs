fn main() {
    let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let mut locks = vec![];
    let mut keys = vec![];
    for thing in input.split("\n\n") {
        let mut grid = vec![];
        for line in thing.lines() {
            grid.push(line.chars().collect::<Vec<_>>());
        }
        let mut vec = vec![];
        for col in 0..grid[0].len() {
            let mut height = 0;
            for row in 0..grid.len() {
                if grid[row][col] == '#' {
                    height += 1;
                }
            }
            vec.push(height - 1);
        }
        if grid[0][0] == '#' {
            locks.push(vec);
        } else {
            keys.push(vec);
        }
    }

    println!("{}", part1(&locks, &keys));
}

fn part1(locks: &[Vec<i32>], keys: &[Vec<i32>]) -> usize {
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|&key| lock.iter().zip(key).all(|(&l, &k)| l + k < 6))
                .count()
        })
        .sum()
}
