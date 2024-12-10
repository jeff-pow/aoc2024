use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader}, time::Instant,
};

fn main() {
    let mut grid = Vec::new();
    let fp = File::open("./example.txt").unwrap();
    let fp = File::open("./input.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        grid.push(
            line.chars()
                .map(|c| {
                    if c == '.' {
                        None
                    } else {
                        Some(c.to_digit(10).unwrap())
                    }
                })
                .collect::<Vec<_>>(),
        );
    }
    let start = Instant::now();
    dbg!(part1(grid));
    dbg!(start.elapsed());
}

fn part2(grid: Vec<Vec<Option<u32>>>) -> usize {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut num_trails = 0;
            if grid[x][y] == Some(0) {
                find_trails_part2(&grid, x, y, &mut num_trails);
                count += num_trails;
            }
        }
    }
    count
}

fn find_trails_part2(grid: &[Vec<Option<u32>>], x: usize, y: usize, num_trails: &mut usize) {
    let Some(current_height) = grid[x][y] else {
        return;
    };

    if current_height == 9 {
        *num_trails += 1;
    }
    if x > 0 && grid[x - 1][y] == Some(current_height + 1) {
        find_trails_part2(grid, x - 1, y, num_trails)
    }
    if x < grid.len() - 1 && grid[x + 1][y] == Some(current_height + 1) {
        find_trails_part2(grid, x + 1, y, num_trails)
    }
    if y > 0 && grid[x][y - 1] == Some(current_height + 1) {
        find_trails_part2(grid, x, y - 1, num_trails)
    }
    if y < grid[0].len() - 1 && grid[x][y + 1] == Some(current_height + 1) {
        find_trails_part2(grid, x, y + 1, num_trails)
    }
}

fn part1(grid: Vec<Vec<Option<u32>>>) -> usize {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut set = Vec::new();
            if grid[x][y] == Some(0) {
                find_trails(&grid, x, y, &mut set);
                count += set.len();
            }
        }
    }
    count
}

fn find_trails(grid: &[Vec<Option<u32>>], x: usize, y: usize, set: &mut Vec<(usize, usize)>) {
    let Some(current_height) = grid[x][y] else {
        return;
    };

    if current_height == 9 && !set.contains(&(x, y)){
        set.push((x, y));
    }
    if x > 0 && grid[x - 1][y] == Some(current_height + 1) {
        find_trails(grid, x - 1, y, set)
    }
    if x < grid.len() - 1 && grid[x + 1][y] == Some(current_height + 1) {
        find_trails(grid, x + 1, y, set)
    }
    if y > 0 && grid[x][y - 1] == Some(current_height + 1) {
        find_trails(grid, x, y - 1, set)
    }

    if y < grid[0].len() - 1 && grid[x][y + 1] == Some(current_height + 1) {
        find_trails(grid, x, y + 1, set)
    }
}
